# src/agents/__init__.py
"""
QRAIOP AI Orchestration System

Multi-agent system for autonomous infrastructure management with
quantum-resistant security and chaos engineering capabilities.
"""

import logging
import asyncio
from typing import Dict, Any, Optional, List
from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from enum import Enum
import json
import time

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)

class AgentRole(Enum):
    """Agent role enumeration"""
    SUPERVISOR = "supervisor"
    SECURITY = "security"
    INFRASTRUCTURE = "infrastructure"
    MONITORING = "monitoring"
    CHAOS = "chaos"

class TaskPriority(Enum):
    """Task priority levels"""
    CRITICAL = 1
    HIGH = 2
    MEDIUM = 3
    LOW = 4

class TaskStatus(Enum):
    """Task execution status"""
    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"
    CANCELLED = "cancelled"

@dataclass
class Task:
    """Task representation for agent execution"""
    id: str
    type: str
    priority: TaskPriority
    data: Dict[str, Any]
    assigned_agent: Optional[str] = None
    status: TaskStatus = TaskStatus.PENDING
    created_at: float = field(default_factory=time.time)
    started_at: Optional[float] = None
    completed_at: Optional[float] = None
    result: Optional[Dict[str, Any]] = None
    error: Optional[str] = None

@dataclass
class AgentState:
    """Agent state representation"""
    agent_id: str
    role: AgentRole
    status: str = "idle"
    last_heartbeat: float = field(default_factory=time.time)
    active_tasks: List[str] = field(default_factory=list)
    capabilities: List[str] = field(default_factory=list)
    metrics: Dict[str, Any] = field(default_factory=dict)

class BaseAgent(ABC):
    """Base class for all QRAIOP agents"""
    
    def __init__(self, agent_id: str, role: AgentRole, config: Dict[str, Any]):
        self.agent_id = agent_id
        self.role = role
        self.config = config
        self.logger = logging.getLogger(f"qraiop.agents.{agent_id}")
        self.state = AgentState(agent_id=agent_id, role=role)
        self.running = False
        self.task_queue = asyncio.Queue()
        
    @abstractmethod
    async def initialize(self) -> None:
        """Initialize the agent"""
        pass
    
    @abstractmethod
    async def process_task(self, task: Task) -> Dict[str, Any]:
        """Process a specific task"""
        pass
    
    @abstractmethod
    async def get_capabilities(self) -> List[str]:
        """Get agent capabilities"""
        pass
    
    async def start(self) -> None:
        """Start the agent"""
        self.logger.info(f"Starting agent {self.agent_id}")
        await self.initialize()
        self.running = True
        self.state.status = "running"
        
        # Start main processing loop
        asyncio.create_task(self._main_loop())
        asyncio.create_task(self._heartbeat_loop())
        
    async def stop(self) -> None:
        """Stop the agent"""
        self.logger.info(f"Stopping agent {self.agent_id}")
        self.running = False
        self.state.status = "stopped"
        
    async def submit_task(self, task: Task) -> None:
        """Submit a task to the agent"""
        task.assigned_agent = self.agent_id
        await self.task_queue.put(task)
        self.state.active_tasks.append(task.id)
        
    async def _main_loop(self) -> None:
        """Main agent processing loop"""
        while self.running:
            try:
                # Wait for task with timeout
                task = await asyncio.wait_for(
                    self.task_queue.get(), timeout=1.0
                )
                
                # Process the task
                await self._execute_task(task)
                
            except asyncio.TimeoutError:
                # No task received, continue
                continue
            except Exception as e:
                self.logger.error(f"Error in main loop: {e}")
                await asyncio.sleep(1)
                
    async def _execute_task(self, task: Task) -> None:
        """Execute a single task"""
        try:
            self.logger.info(f"Executing task {task.id} of type {task.type}")
            task.status = TaskStatus.RUNNING
            task.started_at = time.time()
            
            # Process the task
            result = await self.process_task(task)
            
            # Update task status
            task.status = TaskStatus.COMPLETED
            task.completed_at = time.time()
            task.result = result
            
            # Remove from active tasks
            if task.id in self.state.active_tasks:
                self.state.active_tasks.remove(task.id)
                
            self.logger.info(f"Task {task.id} completed successfully")
            
        except Exception as e:
            self.logger.error(f"Task {task.id} failed: {e}")
            task.status = TaskStatus.FAILED
            task.error = str(e)
            task.completed_at = time.time()
            
            if task.id in self.state.active_tasks:
                self.state.active_tasks.remove(task.id)
                
    async def _heartbeat_loop(self) -> None:
        """Heartbeat loop for agent health monitoring"""
        while self.running:
            self.state.last_heartbeat = time.time()
            await asyncio.sleep(30)  # Heartbeat every 30 seconds
            
    def get_state(self) -> AgentState:
        """Get current agent state"""
        return self.state
        
    def update_metrics(self, metrics: Dict[str, Any]) -> None:
        """Update agent metrics"""
        self.state.metrics.update(metrics)
