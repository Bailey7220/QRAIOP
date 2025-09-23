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
    selector
