# src/chaos/scenarios/network.py
"""
Network chaos scenarios for testing network resilience
"""

from ..chaos_engine import ExperimentConfig, ExperimentTarget, FailureType
from typing import List

def create_network_delay_scenario(
    namespace: str = "default",
    selector: dict = None,
    delay_ms: int = 100,
    duration: int = 60
) -> ExperimentConfig:
    """Create network delay chaos scenario"""
    
    if selector is None:
        selector = {"app": "web"}
        
    return ExperimentConfig(
        name=f"network-delay-{namespace}",
        description=f"Inject {delay_ms}ms network delay for {duration}s",
        failure_type=FailureType.NETWORK_DELAY,
        target=ExperimentTarget(
            namespace=namespace,
            selector=selector,
            percentage=50
        ),
        duration=duration,
        parameters={
            "delay_ms": delay_ms,
            "jitter_ms": delay_ms // 10
        },
        steady_state_hypothesis={
            "pod_health": {
                "namespace": namespace,
                "selector": selector,
                "min_replicas": 1
            },
            "service_availability": {
                "url": f"http://web-service.{namespace}.svc.cluster.local/health",
                "expected_status": 200,
                "timeout": 5
            }
        }
    )

def create_network_partition_scenario(
    namespace: str = "default",
    source_selector: dict = None,
    target_selector: dict = None,
    duration: int = 120
) -> ExperimentConfig:
    """Create network partition chaos scenario"""
    
    if source_selector is None:
        source_selector = {"app": "frontend"}
    if target_selector is None:
        target_selector = {"app": "backend"}
        
    return ExperimentConfig(
        name=f"network-partition-{namespace}",
        description=f"Network partition between services for {duration}s",
        failure_type=FailureType.NETWORK_PARTITION,
        target=ExperimentTarget(
            namespace=namespace,
            selector=source_selector,
            percentage=100
        ),
        duration=duration,
        parameters={
            "target_selector": target_selector,
            "block_ingress": True,
            "block_egress": True
        },
        steady_state_hypothesis={
            "pod_health": {
                "namespace": namespace,
                "selector": source_selector,
                "min_replicas": 1
            }
        }
    )

# Export scenario generators
NETWORK_SCENARIOS = {
    "network_delay": create_network_delay_scenario,
    "network_partition": create_network_partition_scenario
}
