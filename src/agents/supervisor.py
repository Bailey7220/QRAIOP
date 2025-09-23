# src/agents/supervisor.py
"""
QRAIOP Supervisor Agent

Central orchestration agent that coordinates all other agents and manages
the overall system state using LangGraph for workflow orchestration.
"""

import asyncio
import json
from typing import Dict, Any, List, Optional
from dataclasses import dataclass, asdict
from langgraph.graph import StateGraph, START, END
from langgraph.graph.message import add_messages
from langgraph.prebuilt import ToolNode
from langchain_core.messages import BaseMessage, HumanMessage, AIMessage
from langchain_openai import ChatOpenAI
from langchain_anthropic import ChatAnthropic

from . import BaseAgent, AgentRole, Task, TaskPriority, TaskStatus

@dataclass
class SupervisorState:
    """State for the supervisor workflow"""
    messages: List[BaseMessage]
    current_task: Optional[Dict[str, Any]] = None
    assigned_agent: Optional[str] = None
    task_result: Optional[Dict[str, Any]] = None
    system_status: Dict[str, Any] = None
    available_agents: List[str] = None

class SupervisorAgent(BaseAgent):
    """Supervisor agent for orchestrating multi-agent workflows"""
    
    def __init__(self, config: Dict[str, Any]):
        super().__init__("supervisor", AgentRole.SUPERVISOR, config)
        self.agents_registry: Dict[str, BaseAgent] = {}
        self.workflow_graph = None
        self.llm = None
        
    async def initialize(self) -> None:
        """Initialize the supervisor agent"""
        self.logger.info("Initializing Supervisor Agent")
        
        # Initialize LLM
        llm_provider = self.config.get("llm_provider", "openai")
        if llm_provider == "openai":
            self.llm = ChatOpenAI(
                model=self.config.get("llm_model", "gpt-4"),
                api_key=self.config.get("openai_api_key"),
                temperature=0.1
            )
        elif llm_provider == "anthropic":
            self.llm = ChatAnthropic(
                model=self.config.get("llm_model", "claude-3-sonnet-20240229"),
                api_key=self.config.get("anthropic_api_key"),
                temperature=0.1
            )
        
        # Build workflow graph
        self._build_workflow_graph()
        
        self.logger.info("Supervisor Agent initialized successfully")
        
    def _build_workflow_graph(self) -> None:
        """Build the LangGraph workflow for task orchestration"""
        
        def analyze_request(state: SupervisorState) -> SupervisorState:
            """Analyze incoming request and determine required actions"""
            messages = state.messages
            if not messages:
                return state
                
            last_message = messages[-1]
            
            # Use LLM to analyze the request
            system_prompt = """
            You are the QRAIOP Supervisor Agent. Analyze the incoming request and determine:
            1. What type of task needs to be performed
            2. Which agent should handle it (security, infrastructure, monitoring, chaos)
            3. Task priority level (critical, high, medium, low)
            4. Any specific parameters or requirements
            
            Available agents:
            - security: Handles quantum-safe cryptography, security policies, threat detection
            - infrastructure: Manages Kubernetes resources, deployments, scaling
            - monitoring: Collects metrics, monitors system health, alerting
            - chaos: Performs chaos engineering experiments, resilience testing
            
            Respond with a JSON object containing:
            {
                "task_type": "string",
                "assigned_agent": "string", 
                "priority": "critical|high|medium|low",
                "parameters": {}
            }
            """
            
            analysis_messages = [
                HumanMessage(content=system_prompt),
                last_message
            ]
            
            try:
                response = self.llm.invoke(analysis_messages)
                analysis = json.loads(response.content)
                
                state.current_task = analysis
                state.assigned_agent = analysis.get("assigned_agent")
                
            except Exception as e:
                self.logger.error(f"Error analyzing request: {e}")
                state.current_task = {
                    "task_type": "error",
                    "assigned_agent": "supervisor",
                    "priority": "high",
                    "parameters": {"error": str(e)}
                }
                
            return state
            
        def route_to_agent(state: SupervisorState) -> str:
            """Route task to appropriate agent"""
            if not state.current_task:
                return END
                
            assigned_agent = state.assigned_agent
            if assigned_agent in ["security", "infrastructure", "monitoring", "chaos"]:
                return assigned_agent
            else:
                return "supervisor_handle"
                
        def security_agent_node(state: SupervisorState) -> SupervisorState:
            """Security agent processing node"""
            return self._process_agent_task(state, "security")
            
        def infrastructure_agent_node(state: SupervisorState) -> SupervisorState:
            """Infrastructure agent processing node"""
            return self._process_agent_task(state, "infrastructure")
            
        def monitoring_agent_node(state: SupervisorState) -> SupervisorState:
            """Monitoring agent processing node"""
            return self._process_agent_task(state, "monitoring")
            
        def chaos_agent_node(state: SupervisorState) -> SupervisorState:
            """Chaos agent processing node"""
            return self._process_agent_task(state, "chaos")
            
        def supervisor_handle_node(state: SupervisorState) -> SupervisorState:
            """Handle tasks that supervisor manages directly"""
            task = state.current_task
            
            if task.get("task_type") == "system_status":
                # Return system status
                state.task_result = {
                    "status": "healthy",
                    "agents": list(self.agents_registry.keys()),
                    "active_tasks": len(self.state.active_tasks),
                    "uptime": time.time() - self.state.last_heartbeat
                }
            elif task.get("task_type") == "error":
                state.task_result = {
                    "error": task.get("parameters", {}).get("error", "Unknown error")
                }
            else:
                state.task_result = {
                    "message": "Task handled by supervisor",
                    "task": task
                }
                
            return state
            
        def finalize_response(state: SupervisorState) -> SupervisorState:
            """Finalize response and add to messages"""
            if state.task_result:
                response_msg = AIMessage(
                    content=json.dumps(state.task_result, indent=2)
                )
                state.messages.append(response_msg)
                
            return state
        
        # Build the workflow graph
        workflow = StateGraph(SupervisorState)
        
        # Add nodes
        workflow.add_node("analyze", analyze_request)
        workflow.add_node("security", security_agent_node)
        workflow.add_node("infrastructure", infrastructure_agent_node)
        workflow.add_node("monitoring", monitoring_agent_node)
        workflow.add_node("chaos", chaos_agent_node)
        workflow.add_node("supervisor_handle", supervisor_handle_node)
        workflow.add_node("finalize", finalize_response)
        
        # Add edges
        workflow.add_edge(START, "analyze")
        workflow.add_conditional_edges(
            "analyze",
            route_to_agent,
            {
                "security": "security",
                "infrastructure": "infrastructure", 
                "monitoring": "monitoring",
                "chaos": "chaos",
                "supervisor_handle": "supervisor_handle",
                END: END
            }
        )
        
        # All specialized nodes go to finalize
        workflow.add_edge("security", "finalize")
        workflow.add_edge("infrastructure", "finalize")
        workflow.add_edge("monitoring", "finalize")
        workflow.add_edge("chaos", "finalize")
        workflow.add_edge("supervisor_handle", "finalize")
        workflow.add_edge("finalize", END)
        
        self.workflow_graph = workflow.compile()
        
    def _process_agent_task(self, state: SupervisorState, agent_type: str) -> SupervisorState:
        """Process task with specified agent"""
        if agent_type not in self.agents_registry:
            state.task_result = {
                "error": f"Agent {agent_type} not available"
            }
            return state
            
        try:
            # Create task for the agent
            task = Task(
                id=f"task_{int(time.time())}",
                type=state.current_task.get("task_type", "unknown"),
                priority=TaskPriority.HIGH,
                data=state.current_task.get("parameters", {})
            )
            
            # Submit task to agent (simplified for synchronous processing)
            agent = self.agents_registry[agent_type]
            
            # For demo purposes, create a mock result
            state.task_result = {
                "agent": agent_type,
                "task_id": task.id,
                "status": "completed",
                "message": f"Task processed by {agent_type} agent"
            }
            
        except Exception as e:
            state.task_result = {
                "error": f"Error processing task with {agent_type}: {str(e)}"
            }
            
        return state
        
    async def process_task(self, task: Task) -> Dict[str, Any]:
        """Process a task using the workflow graph"""
        try:
            # Convert task to workflow state
            initial_state = SupervisorState(
                messages=[HumanMessage(content=json.dumps(asdict(task)))],
                system_status={"healthy": True},
                available_agents=list(self.agents_registry.keys())
            )
            
            # Run the workflow
            result = await asyncio.get_event_loop().run_in_executor(
                None, 
                lambda: self.workflow_graph.invoke(initial_state)
            )
            
            return {
                "status": "completed",
                "result": result.task_result,
                "messages": [msg.content for msg in result.messages]
            }
            
        except Exception as e:
            self.logger.error(f"Error processing task: {e}")
            return {
                "status": "failed",
                "error": str(e)
            }
            
    async def get_capabilities(self) -> List[str]:
        """Get supervisor capabilities"""
        return [
            "task_orchestration",
            "agent_coordination", 
            "workflow_management",
            "system_monitoring",
            "decision_making",
            "multi_agent_communication"
        ]
        
    def register_agent(self, agent: BaseAgent) -> None:
        """Register an agent with the supervisor"""
        self.agents_registry[agent.agent_id] = agent
        self.logger.info(f"Registered agent: {agent.agent_id}")
        
    def unregister_agent(self, agent_id: str) -> None:
        """Unregister an agent"""
        if agent_id in self.agents_registry:
            del self.agents_registry[agent_id]
            self.logger.info(f"Unregistered agent: {agent_id}")
            
    async def get_system_status(self) -> Dict[str, Any]:
        """Get overall system status"""
        agent_statuses = {}
        for agent_id, agent in self.agents_registry.items():
            agent_statuses[agent_id] = {
                "role": agent.role.value,
                "status": agent.state.status,
                "active_tasks": len(agent.state.active_tasks),
                "last_heartbeat": agent.state.last_heartbeat,
                "capabilities": await agent.get_capabilities()
            }
            
        return {
            "supervisor": {
                "status": self.state.status,
                "registered_agents": len(self.agents_registry),
                "total_active_tasks": sum(len(a.state.active_tasks) for a in self.agents_registry.values())
            },
            "agents": agent_statuses
        }

# Example usage and testing
if __name__ == "__main__":
    import time
    
    async def main():
        config = {
            "llm_provider": "openai",
            "llm_model": "gpt-4",
            "openai_api_key": "your-api-key-here"
        }
        
        supervisor = SupervisorAgent(config)
        await supervisor.start()
        
        # Test task
        test_task = Task(
            id="test_001",
            type="security_audit",
            priority=TaskPriority.HIGH,
            data={"target": "kubernetes_cluster"}
        )
        
        result = await supervisor.process_task(test_task)
        print("Task result:", json.dumps(result, indent=2))
        
        await supervisor.stop()
        
    asyncio.run(main())
