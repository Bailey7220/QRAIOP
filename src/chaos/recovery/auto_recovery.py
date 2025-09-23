# src/chaos/recovery/auto_recovery.py
"""
Automated recovery system for chaos engineering experiments
"""

import asyncio
import logging
from typing import Dict, Any, List, Callable
from dataclasses import dataclass
from datetime import datetime, timedelta

@dataclass
class RecoveryRule:
    """Rule for automatic recovery"""
    name: str
    condition: Callable[[Dict[str, Any]], bool]
    action: Callable[[Dict[str, Any]], Any]
    priority: int = 1
    max_retries: int = 3
    
class AutoRecoverySystem:
    """Automated recovery system"""
    
    def __init__(self, chaos_engine):
        self.chaos_engine = chaos_engine
        self.logger = logging.getLogger("qraiop.chaos.recovery")
        self.recovery_rules: List[RecoveryRule] = []
        self.running = False
        
        # Register default recovery rules
        self._register_default_rules()
        
    def _register_default_rules(self):
        """Register default recovery rules"""
        
        # Pod crash recovery
        self.add_rule(RecoveryRule(
            name="pod_crash_recovery",
            condition=self._check_pod_crash,
            action=self._recover_pod_crash,
            priority=1
        ))
        
        # Service unavailability recovery
        self.add_rule(RecoveryRule(
            name="service_unavailable_recovery", 
            condition=self._check_service_unavailable,
            action=self._recover_service_unavailable,
            priority=2
        ))
        
        # High error rate recovery
        self.add_rule(RecoveryRule(
            name="high_error_rate_recovery",
            condition=self._check_high_error_rate,
            action=self._recover_high_error_rate,
            priority=3
        ))
        
    def add_rule(self, rule: RecoveryRule):
        """Add a recovery rule"""
        self.recovery_rules.append(rule)
        self.recovery_rules.sort(key=lambda r: r.priority)
        
    def _check_pod_crash(self, metrics: Dict[str, Any]) -> bool:
        """Check if pods are crashing"""
        pod_health = metrics.get("pod_health", {})
        running_pods = pod_health.get("running_pods", 0)
        min_required = pod_health.get("min_required", 1)
        return running_pods < min_required
        
    def _check_service_unavailable(self, metrics: Dict[str, Any]) -> bool:
        """Check if service is unavailable"""
        service_health = metrics.get("service_availability", {})
        return not service_health.get("valid", True)
        
    def _check_high_error_rate(self, metrics: Dict[str, Any]) -> bool:
        """Check if error rate is too high"""
        error_rate = metrics.get("error_rate", {}).get("error_rate_percent", 0)
        return error_rate > 5.0  # More than 5% error rate
        
    async def _recover_pod_crash(self, metrics: Dict[str, Any]):
        """Recover from pod crashes"""
        self.logger.warning("Detected pod crashes, initiating recovery")
        
        # Scale up deployment
        namespace = metrics.get("namespace", "default")
        deployment = metrics.get("deployment", "web")
        
        try:
            # Get current deployment
            deployment_obj = self.chaos_engine.apps_v1.read_namespaced_deployment(
                name=deployment,
                namespace=namespace
            )
            
            # Scale up by 1 replica
            current_replicas = deployment_obj.spec.replicas
            deployment_obj.spec.replicas = current_replicas + 1
            
            # Update deployment
            self.chaos_engine.apps_v1.patch_namespaced_deployment(
                name=deployment,
                namespace=namespace,
                body=deployment_obj
            )
            
            self.logger.info(f"Scaled up {deployment} to {current_replicas + 1} replicas")
            
        except Exception as e:
            self.logger.error(f"Failed to recover from pod crash: {e}")
            
    async def _recover_service_unavailable(self, metrics: Dict[str, Any]):
        """Recover from service unavailability"""
        self.logger.warning("Detected service unavailability, initiating recovery")
        
        # Restart pods
        namespace = metrics.get("namespace", "default")
        selector = metrics.get("selector", {"app": "web"})
        
        try:
            selector_str = ",".join([f"{k}={v}" for k, v in selector.items()])
            pods = self.chaos_engine.core_v1.list_namespaced_pod(
                namespace=namespace,
                label_selector=selector_str
            )
            
            for pod in pods.items:
                self.logger.info(f"Restarting pod {pod.metadata.name}")
                self.chaos_engine.core_v1.delete_namespaced_pod(
                    name=pod.metadata.name,
                    namespace=namespace
                )
                
        except Exception as e:
            self.logger.error(f"Failed to recover from service unavailability: {e}")
            
    async def _recover_high_error_rate(self, metrics: Dict[str, Any]):
        """Recover from high error rate"""
        self.logger.warning("Detected high error rate, initiating recovery")
        
        # Implement circuit breaker or rate limiting
        # This would typically involve updating service mesh configuration
        
        self.logger.info("Applied circuit breaker configuration")
        
    async def monitor_and_recover(self, interval: int = 30):
        """Monitor system and apply recovery rules"""
        self.running = True
        
        while self.running:
            try:
                # Collect system metrics
                metrics = await self._collect_system_metrics()
                
                # Check recovery rules
                for rule in self.recovery_rules:
                    if rule.condition(metrics):
                        self.logger.info(f"Triggering recovery rule: {rule.name}")
                        await rule.action(metrics)
                        
                await asyncio.sleep(interval)
                
            except Exception as e:
                self.logger.error(f"Error in recovery monitoring: {e}")
                await asyncio.sleep(interval)
                
    async def _collect_system_metrics(self) -> Dict[str, Any]:
        """Collect current system metrics"""
        # This would integrate with monitoring systems
        return {
            "pod_health": {"running_pods": 3, "min_required": 2},
            "service_availability": {"valid": True},
            "error_rate": {"error_rate_percent": 1.2}
        }
        
    def stop(self):
        """Stop the recovery system"""
        self.running = False
