# src/chaos/chaos_engine.py
"""
QRAIOP Chaos Engineering Engine

Intelligent chaos engineering platform that automatically discovers,
tests, and validates system resilience through controlled failure injection.
"""

import asyncio
import json
import logging
import time
import uuid
from datetime import datetime, timedelta
from typing import Dict, Any, List, Optional, Callable
from dataclasses import dataclass, field, asdict
from enum import Enum
import yaml

from kubernetes import client, config
from prometheus_client.parser import text_string_to_metric_families
import requests
import psutil

class ExperimentStatus(Enum):
    """Chaos experiment status"""
    PENDING = "pending"
    RUNNING = "running"  
    COMPLETED = "completed"
    FAILED = "failed"
    ABORTED = "aborted"

class FailureType(Enum):
    """Types of failures to inject"""
    POD_KILL = "pod_kill"
    NETWORK_DELAY = "network_delay"
    NETWORK_PARTITION = "network_partition"
    CPU_STRESS = "cpu_stress"
    MEMORY_STRESS = "memory_stress"
    DISK_FILL = "disk_fill"
    DNS_CHAOS = "dns_chaos"
    SERVICE_MESH_FAULT = "service_mesh_fault"

@dataclass
class ExperimentTarget:
    """Target for chaos experiment"""
    namespace: str
    selector: Dict[str, str]
    percentage: int = 100  # Percentage of targets to affect
    
@dataclass  
class ExperimentConfig:
    """Configuration for a chaos experiment"""
    name: str
    description: str
    failure_type: FailureType
    target: ExperimentTarget
    duration: int  # Duration in seconds
    parameters: Dict[str, Any] = field(default_factory=dict)
    steady_state_hypothesis: Optional[Dict[str, Any]] = None
    rollback_config: Optional[Dict[str, Any]] = None
    
@dataclass
class ExperimentResult:
    """Result of a chaos experiment"""
    experiment_id: str
    name: str
    status: ExperimentStatus
    start_time: datetime
    end_time: Optional[datetime] = None
    duration: Optional[int] = None
    steady_state_before: Optional[Dict[str, Any]] = None
    steady_state_after: Optional[Dict[str, Any]] = None
    injected_failures: List[Dict[str, Any]] = field(default_factory=list)
    recovery_actions: List[Dict[str, Any]] = field(default_factory=list)
    metrics: Dict[str, Any] = field(default_factory=dict)
    error_message: Optional[str] = None

class ChaosEngine:
    """Main chaos engineering engine"""
    
    def __init__(self, config: Dict[str, Any]):
        self.config = config
        self.logger = logging.getLogger("qraiop.chaos.engine")
        self.k8s_client = None
        self.running_experiments: Dict[str, ExperimentResult] = {}
        self.experiment_history: List[ExperimentResult] = []
        
        # Initialize Kubernetes client
        self._init_kubernetes()
        
    def _init_kubernetes(self) -> None:
        """Initialize Kubernetes client"""
        try:
            # Try in-cluster config first
            config.load_incluster_config()
            self.logger.info("Loaded in-cluster Kubernetes config")
        except:
            try:
                # Fall back to kubeconfig
                config.load_kube_config()
                self.logger.info("Loaded kubeconfig")
            except Exception as e:
                self.logger.error(f"Failed to load Kubernetes config: {e}")
                raise
                
        self.k8s_client = client.ApiClient()
        self.apps_v1 = client.AppsV1Api()
        self.core_v1 = client.CoreV1Api()
        
    async def run_experiment(self, experiment_config: ExperimentConfig) -> ExperimentResult:
        """Run a chaos experiment"""
        experiment_id = str(uuid.uuid4())
        
        result = ExperimentResult(
            experiment_id=experiment_id,
            name=experiment_config.name,
            status=ExperimentStatus.PENDING,
            start_time=datetime.now()
        )
        
        self.running_experiments[experiment_id] = result
        
        try:
            self.logger.info(f"Starting chaos experiment: {experiment_config.name}")
            
            # Validate steady state before experiment
            if experiment_config.steady_state_hypothesis:
                result.steady_state_before = await self._validate_steady_state(
                    experiment_config.steady_state_hypothesis
                )
                
                if not result.steady_state_before.get("valid", False):
                    raise Exception("Steady state validation failed before experiment")
            
            result.status = ExperimentStatus.RUNNING
            
            # Inject the failure
            failure_info = await self._inject_failure(experiment_config)
            result.injected_failures.append(failure_info)
            
            # Wait for experiment duration
            await asyncio.sleep(experiment_config.duration)
            
            # Collect metrics during experiment
            result.metrics = await self._collect_metrics(experiment_config)
            
            # Clean up and recover
            recovery_info = await self._recover_from_failure(experiment_config, failure_info)
            result.recovery_actions.append(recovery_info)
            
            # Validate steady state after experiment
            if experiment_config.steady_state_hypothesis:
                result.steady_state_after = await self._validate_steady_state(
                    experiment_config.steady_state_hypothesis
                )
            
            result.status = ExperimentStatus.COMPLETED
            result.end_time = datetime.now()
            result.duration = int((result.end_time - result.start_time).total_seconds())
            
            self.logger.info(f"Chaos experiment {experiment_config.name} completed successfully")
            
        except Exception as e:
            self.logger.error(f"Chaos experiment {experiment_config.name} failed: {e}")
            result.status = ExperimentStatus.FAILED
            result.error_message = str(e)
            result.end_time = datetime.now()
            
            # Attempt recovery even if experiment failed
            try:
                if result.injected_failures:
                    recovery_info = await self._recover_from_failure(
                        experiment_config, 
                        result.injected_failures[-1]
                    )
                    result.recovery_actions.append(recovery_info)
            except Exception as recovery_error:
                self.logger.error(f"Recovery failed: {recovery_error}")
                
        finally:
            # Move to history and clean up
            self.experiment_history.append(result)
            if experiment_id in self.running_experiments:
                del self.running_experiments[experiment_id]
                
        return result
        
    async def _inject_failure(self, config: ExperimentConfig) -> Dict[str, Any]:
        """Inject specific type of failure"""
        failure_type = config.failure_type
        
        if failure_type == FailureType.POD_KILL:
            return await self._inject_pod_kill(config)
        elif failure_type == FailureType.NETWORK_DELAY:
            return await self._inject_network_delay(config)
        elif failure_type == FailureType.NETWORK_PARTITION:
            return await self._inject_network_partition(config)
        elif failure_type == FailureType.CPU_STRESS:
            return await self._inject_cpu_stress(config)
        elif failure_type == FailureType.MEMORY_STRESS:
            return await self._inject_memory_stress(config)
        else:
            raise NotImplementedError(f"Failure type {failure_type} not implemented")
            
    async def _inject_pod_kill(self, config: ExperimentConfig) -> Dict[str, Any]:
        """Kill pods matching the target selector"""
        namespace = config.target.namespace
        selector = ",".join([f"{k}={v}" for k, v in config.target.selector.items()])
        
        try:
            # Get pods matching selector
            pods = self.core_v1.list_namespaced_pod(
                namespace=namespace,
                label_selector=selector
            )
            
            if not pods.items:
                raise Exception(f"No pods found with selector {selector} in namespace {namespace}")
                
            # Calculate number of pods to kill based on percentage
            num_to_kill = max(1, int(len(pods.items) * config.target.percentage / 100))
            pods_to_kill = pods.items[:num_to_kill]
            
            killed_pods = []
            for pod in pods_to_kill:
                self.logger.info(f"Killing pod {pod.metadata.name}")
                self.core_v1.delete_namespaced_pod(
                    name=pod.metadata.name,
                    namespace=namespace
                )
                killed_pods.append(pod.metadata.name)
                
            return {
                "type": "pod_kill",
                "namespace": namespace,
                "selector": selector,
                "killed_pods": killed_pods,
                "timestamp": datetime.now().isoformat()
            }
            
        except Exception as e:
            self.logger.error(f"Failed to kill pods: {e}")
            raise
            
    async def _inject_network_delay(self, config: ExperimentConfig) -> Dict[str, Any]:
        """Inject network delay using tc (traffic control)"""
        delay_ms = config.parameters.get("delay_ms", 100)
        jitter_ms = config.parameters.get("jitter_ms", 10)
        
        # Create network chaos using Kubernetes NetworkPolicy or external tools
        # This is a simplified implementation
        return {
            "type": "network_delay",
            "delay_ms": delay_ms,
            "jitter_ms": jitter_ms,
            "target": asdict(config.target),
            "timestamp": datetime.now().isoformat()
        }
        
    async def _inject_network_partition(self, config: ExperimentConfig) -> Dict[str, Any]:
        """Create network partition between services"""
        # Implementation would use iptables or similar to block traffic
        return {
            "type": "network_partition", 
            "target": asdict(config.target),
            "timestamp": datetime.now().isoformat()
        }
        
    async def _inject_cpu_stress(self, config: ExperimentConfig) -> Dict[str, Any]:
        """Inject CPU stress on target pods"""
        cpu_percent = config.parameters.get("cpu_percent", 80)
        
        # Use stress-ng or similar tool in target pods
        return {
            "type": "cpu_stress",
            "cpu_percent": cpu_percent,
            "target": asdict(config.target),
            "timestamp": datetime.now().isoformat()
        }
        
    async def _inject_memory_stress(self, config: ExperimentConfig) -> Dict[str, Any]:
        """Inject memory stress on target pods"""
        memory_mb = config.parameters.get("memory_mb", 1024)
        
        return {
            "type": "memory_stress",
            "memory_mb": memory_mb,
            "target": asdict(config.target),
            "timestamp": datetime.now().isoformat()
        }
        
    async def _recover_from_failure(self, config: ExperimentConfig, failure_info: Dict[str, Any]) -> Dict[str, Any]:
        """Recover from injected failure"""
        failure_type = failure_info["type"]
        
        if failure_type == "pod_kill":
            # Pods should auto-recover via deployments
            return await self._wait_for_pod_recovery(config, failure_info)
        elif failure_type in ["network_delay", "network_partition"]:
            return await self._recover_network(config, failure_info)
        elif failure_type in ["cpu_stress", "memory_stress"]:
            return await self._recover_resource_stress(config, failure_info)
        else:
            return {"type": "no_recovery_needed", "timestamp": datetime.now().isoformat()}
            
    async def _wait_for_pod_recovery(self, config: ExperimentConfig, failure_info: Dict[str, Any]) -> Dict[str, Any]:
        """Wait for killed pods to be recreated"""
        namespace = config.target.namespace
        selector = ",".join([f"{k}={v}" for k, v in config.target.selector.items()])
        
        recovery_start = time.time()
        timeout = 300  # 5 minutes timeout
        
        while time.time() - recovery_start < timeout:
            pods = self.core_v1.list_namespaced_pod(
                namespace=namespace,
                label_selector=selector
            )
            
            # Check if all pods are running
            running_pods = [p for p in pods.items if p.status.phase == "Running"]
            if len(running_pods) >= len(failure_info["killed_pods"]):
                recovery_time = time.time() - recovery_start
                return {
                    "type": "pod_recovery",
                    "recovery_time_seconds": recovery_time,
                    "recovered_pods": [p.metadata.name for p in running_pods],
                    "timestamp": datetime.now().isoformat()
                }
                
            await asyncio.sleep(5)
            
        raise Exception("Pod recovery timeout")
        
    async def _recover_network(self, config: ExperimentConfig, failure_info: Dict[str, Any]) -> Dict[str, Any]:
        """Recover from network failures"""
        # Remove network policies or iptables rules
        return {
            "type": "network_recovery",
            "timestamp": datetime.now().isoformat()
        }
        
    async def _recover_resource_stress(self, config: ExperimentConfig, failure_info: Dict[str, Any]) -> Dict[str, Any]:
        """Recover from resource stress"""
        # Kill stress processes
        return {
            "type": "resource_recovery", 
            "timestamp": datetime.now().isoformat()
        }
        
    async def _validate_steady_state(self, hypothesis: Dict[str, Any]) -> Dict[str, Any]:
        """Validate system steady state"""
        # Check various system health indicators
        checks = []
        
        # Check pod health
        if "pod_health" in hypothesis:
            pod_check = await self._check_pod_health(hypothesis["pod_health"])
            checks.append(pod_check)
            
        # Check service availability  
        if "service_availability" in hypothesis:
            service_check = await self._check_service_availability(hypothesis["service_availability"])
            checks.append(service_check)
            
        # Check custom metrics
        if "metrics" in hypothesis:
            metrics_check = await self._check_metrics(hypothesis["metrics"])
            checks.append(metrics_check)
            
        all_valid = all(check["valid"] for check in checks)
        
        return {
            "valid": all_valid,
            "checks": checks,
            "timestamp": datetime.now().isoformat()
        }
        
    async def _check_pod_health(self, config: Dict[str, Any]) -> Dict[str, Any]:
        """Check pod health status"""
        namespace = config.get("namespace", "default")
        selector = config.get("selector", {})
        min_replicas = config.get("min_replicas", 1)
        
        selector_str = ",".join([f"{k}={v}" for k, v in selector.items()])
        
        try:
            pods = self.core_v1.list_namespaced_pod(
                namespace=namespace,
                label_selector=selector_str
            )
            
            running_pods = [p for p in pods.items if p.status.phase == "Running"]
            ready_pods = [p for p in running_pods if all(
                condition.status == "True" 
                for condition in (p.status.conditions or [])
                if condition.type == "Ready"
            )]
            
            valid = len(ready_pods) >= min_replicas
            
            return {
                "type": "pod_health",
                "valid": valid,
                "running_pods": len(running_pods),
                "ready_pods": len(ready_pods),
                "min_required": min_replicas
            }
            
        except Exception as e:
            return {
                "type": "pod_health",
                "valid": False,
                "error": str(e)
            }
            
    async def _check_service_availability(self, config: Dict[str, Any]) -> Dict[str, Any]:
        """Check service endpoint availability"""
        url = config.get("url")
        timeout = config.get("timeout", 5)
        expected_status = config.get("expected_status", 200)
        
        try:
            response = requests.get(url, timeout=timeout)
            valid = response.status_code == expected_status
            
            return {
                "type": "service_availability",
                "valid": valid,
                "status_code": response.status_code,
                "response_time": response.elapsed.total_seconds()
            }
            
        except Exception as e:
            return {
                "type": "service_availability",
                "valid": False,
                "error": str(e)
            }
            
    async def _check_metrics(self, config: Dict[str, Any]) -> Dict[str, Any]:
        """Check custom metrics thresholds"""
        # Integration with Prometheus or other metrics systems
        return {
            "type": "metrics",
            "valid": True,  # Placeholder
            "metrics": config
        }
        
    async def _collect_metrics(self, config: ExperimentConfig) -> Dict[str, Any]:
        """Collect metrics during experiment"""
        return {
            "cpu_usage": await self._get_cpu_metrics(config),
            "memory_usage": await self._get_memory_metrics(config), 
            "network_latency": await self._get_network_metrics(config),
            "error_rate": await self._get_error_rate_metrics(config)
        }
        
    async def _get_cpu_metrics(self, config: ExperimentConfig) -> Dict[str, Any]:
        """Get CPU usage metrics"""
        return {"avg_cpu_percent": 45.2}  # Placeholder
        
    async def _get_memory_metrics(self, config: ExperimentConfig) -> Dict[str, Any]:
        """Get memory usage metrics"""
        return {"avg_memory_percent": 67.1}  # Placeholder
        
    async def _get_network_metrics(self, config: ExperimentConfig) -> Dict[str, Any]:
        """Get network latency metrics"""
        return {"avg_latency_ms": 23.4}  # Placeholder
        
    async def _get_error_rate_metrics(self, config: ExperimentConfig) -> Dict[str, Any]:
        """Get error rate metrics"""
        return {"error_rate_percent": 0.1}  # Placeholder
        
    def get_experiment_status(self, experiment_id: str) -> Optional[ExperimentResult]:
        """Get status of running or completed experiment"""
        if experiment_id in self.running_experiments:
            return self.running_experiments[experiment_id]
            
        for result in self.experiment_history:
            if result.experiment_id == experiment_id:
                return result
                
        return None
        
    def list_experiments(self, status: Optional[ExperimentStatus] = None) -> List[ExperimentResult]:
        """List experiments, optionally filtered by status"""
        all_experiments = list(self.running_experiments.values()) + self.experiment_history
        
        if status:
            return [exp for exp in all_experiments if exp.status == status]
            
        return all_experiments
        
    async def abort_experiment(self, experiment_id: str) -> bool:
        """Abort a running experiment"""
        if experiment_id not in self.running_experiments:
            return False
            
        result = self.running_experiments[experiment_id]
        result.status = ExperimentStatus.ABORTED
        result.end_time = datetime.now()
        
        # Attempt recovery
        try:
            if result.injected_failures:
                # Recover from the last injected failure
                failure_info = result.injected_failures[-1]
                # Create temporary config for recovery
                temp_config = ExperimentConfig(
                    name="recovery",
                    description="Recovery from aborted experiment",
                    failure_type=FailureType.POD_KILL,  # Will be overridden
                    target=ExperimentTarget(namespace="default", selector={})
                )
                recovery_info = await self._recover_from_failure(temp_config, failure_info)
                result.recovery_actions.append(recovery_info)
        except Exception as e:
            self.logger.error(f"Failed to recover from aborted experiment: {e}")
            
        # Move to history
        self.experiment_history.append(result)
        del self.running_experiments[experiment_id]
        
        return True
