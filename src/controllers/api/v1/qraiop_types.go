// src/controllers/api/v1/qraiop_types.go
package v1

import (
    metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
)

// QraiopSpec defines the desired state of Qraiop
type QraiopSpec struct {
    // Quantum-safe cryptography configuration
    Cryptography CryptographyConfig `json:"cryptography,omitempty"`
    
    // AI orchestration configuration
    AIOrchestration AIConfig `json:"aiOrchestration,omitempty"`
    
    // Chaos engineering configuration
    ChaosEngineering ChaosConfig `json:"chaosEngineering,omitempty"`
    
    // Monitoring configuration
    Monitoring MonitoringConfig `json:"monitoring,omitempty"`
    
    // Security policies
    SecurityPolicies SecurityConfig `json:"securityPolicies,omitempty"`
}

// CryptographyConfig defines quantum-safe crypto settings
type CryptographyConfig struct {
    // Enable post-quantum cryptography
    Enabled bool `json:"enabled"`
    
    // Supported algorithms
    Algorithms []string `json:"algorithms,omitempty"`
    
    // Security level (1, 3, or 5)
    SecurityLevel int `json:"securityLevel,omitempty"`
    
    // Hybrid mode (classical + quantum-safe)
    HybridMode bool `json:"hybridMode,omitempty"`
    
    // Certificate management
    CertificateManagement CertManagementConfig `json:"certificateManagement,omitempty"`
}

// CertManagementConfig defines certificate management
type CertManagementConfig struct {
    // Enable automatic certificate rotation
    AutoRotation bool `json:"autoRotation,omitempty"`
    
    // Rotation interval in hours
    RotationInterval int `json:"rotationInterval,omitempty"`
    
    // Certificate authority
    CertificateAuthority string `json:"certificateAuthority,omitempty"`
}

// AIConfig defines AI orchestration settings
type AIConfig struct {
    // Enable AI orchestration
    Enabled bool `json:"enabled"`
    
    // LLM provider (openai, anthropic, local)
    LLMProvider string `json:"llmProvider,omitempty"`
    
    // Model configuration
    ModelConfig ModelConfig `json:"modelConfig,omitempty"`
    
    // Agent configuration
    Agents []AgentConfig `json:"agents,omitempty"`
}

// ModelConfig defines LLM model settings
type ModelConfig struct {
    // Model name
    Model string `json:"model,omitempty"`
    
    // Temperature for model responses
    Temperature float32 `json:"temperature,omitempty"`
    
    // Maximum tokens
    MaxTokens int `json:"maxTokens,omitempty"`
}

// AgentConfig defines individual agent settings
type AgentConfig struct {
    // Agent type (supervisor, security, infrastructure, monitoring, chaos)
    Type string `json:"type"`
    
    // Enable the agent
    Enabled bool `json:"enabled"`
    
    // Agent-specific configuration
    Config map[string]string `json:"config,omitempty"`
}

// ChaosConfig defines chaos engineering settings
type ChaosConfig struct {
    // Enable chaos engineering
    Enabled bool `json:"enabled"`
    
    // Experiment schedules
    Schedules []ChaosSchedule `json:"schedules,omitempty"`
    
    // Safety configuration
    Safety ChaosSafetyConfig `json:"safety,omitempty"`
}

// ChaosSchedule defines scheduled chaos experiments
type ChaosSchedule struct {
    // Schedule name
    Name string `json:"name"`
    
    // Cron schedule
    Schedule string `json:"schedule"`
    
    // Experiment configuration
    ExperimentConfig map[string]interface{} `json:"experimentConfig"`
}

// ChaosSafetyConfig defines chaos safety settings
type ChaosSafetyConfig struct {
    // Maximum concurrent experiments
    MaxConcurrentExperiments int `json:"maxConcurrentExperiments,omitempty"`
    
    // Excluded namespaces
    ExcludedNamespaces []string `json:"excludedNamespaces,omitempty"`
    
    // Business hours only
    BusinessHoursOnly bool `json:"businessHoursOnly,omitempty"`
}

// MonitoringConfig defines monitoring settings
type MonitoringConfig struct {
    // Enable monitoring
    Enabled bool `json:"enabled"`
    
    // Prometheus configuration
    Prometheus PrometheusConfig `json:"prometheus,omitempty"`
    
    // Grafana configuration
    Grafana GrafanaConfig `json:"grafana,omitempty"`
    
    // Alerting configuration
    Alerting AlertingConfig `json:"alerting,omitempty"`
}

// PrometheusConfig defines Prometheus settings
type PrometheusConfig struct {
    // Enable Prometheus
    Enabled bool `json:"enabled"`
    
    // Scrape interval
    ScrapeInterval string `json:"scrapeInterval,omitempty"`
    
    // Retention period
    Retention string `json:"retention,omitempty"`
}

// GrafanaConfig defines Grafana settings
type GrafanaConfig struct {
    // Enable Grafana
    Enabled bool `json:"enabled"`
    
    // Dashboard provisioning
    DashboardProvisioning bool `json:"dashboardProvisioning,omitempty"`
}

// AlertingConfig defines alerting settings
type AlertingConfig struct {
    // Enable alerting
    Enabled bool `json:"enabled"`
    
    // Alert channels
    Channels []AlertChannel `json:"channels,omitempty"`
}

// AlertChannel defines alert channel configuration
type AlertChannel struct {
    // Channel type (slack, email, webhook)
    Type string `json:"type"`
    
    // Channel configuration
    Config map[string]string `json:"config"`
}

// SecurityConfig defines security policy settings
type SecurityConfig struct {
    // Network policies
    NetworkPolicies NetworkPolicyConfig `json:"networkPolicies,omitempty"`
    
    // Pod security standards
    PodSecurityStandards PodSecurityConfig `json:"podSecurityStandards,omitempty"`
    
    // RBAC configuration
    RBAC RBACConfig `json:"rbac,omitempty"`
}

// NetworkPolicyConfig defines network policy settings
type NetworkPolicyConfig struct {
    // Enable default deny-all policy
    DefaultDenyAll bool `json:"defaultDenyAll,omitempty"`
    
    // Allow QRAIOP components communication
    AllowQraiopCommunication bool `json:"allowQraiopCommunication,omitempty"`
}

// PodSecurityConfig defines pod security settings
type PodSecurityConfig struct {
    // Security level (privileged, baseline, restricted)
    Level string `json:"level,omitempty"`
    
    // Enforce security standards
    Enforce bool `json:"enforce,omitempty"`
}

// RBACConfig defines RBAC settings
type RBACConfig struct {
    // Enable RBAC
    Enabled bool `json:"enabled"`
    
    // Service account configuration
    ServiceAccounts []ServiceAccountConfig `json:"serviceAccounts,omitempty"`
}

// ServiceAccountConfig defines service account settings
type ServiceAccountConfig struct {
    // Service account name
    Name string `json:"name"`
    
    // Namespace
    Namespace string `json:"namespace"`
    
    // Roles
    Roles []string `json:"roles,omitempty"`
    
    // Cluster roles
    ClusterRoles []string `json:"clusterRoles,omitempty"`
}

// QraiopStatus defines the observed state of Qraiop
type QraiopStatus struct {
    // Overall status
    Phase string `json:"phase,omitempty"`
    
    // Status message
    Message string `json:"message,omitempty"`
    
    // Component statuses
    Components map[string]ComponentStatus `json:"components,omitempty"`
    
    // Last update timestamp
    LastUpdated metav1.Time `json:"lastUpdated,omitempty"`
    
    // Conditions
    Conditions []metav1.Condition `json:"conditions,omitempty"`
}

// ComponentStatus defines individual component status
type ComponentStatus struct {
    // Component status (Ready, NotReady, Error)
    Status string `json:"status"`
    
    // Status message
    Message string `json:"message,omitempty"`
    
    // Last update timestamp
    LastUpdated metav1.Time `json:"lastUpdated,omitempty"`
}

//+kubebuilder:object:root=true
//+kubebuilder:subresource:status
//+kubebuilder:resource:scope=Namespaced,shortName=qraiop
//+kubebuilder:printcolumn:name="Phase",type=string,JSONPath=".status.phase"
//+kubebuilder:printcolumn:name="Age",type=date,JSONPath=".metadata.creationTimestamp"

// Qraiop is the Schema for the qraiops API
type Qraiop struct {
    metav1.TypeMeta   `json:",inline"`
    metav1.ObjectMeta `json:"metadata,omitempty"`

    Spec   QraiopSpec   `json:"spec,omitempty"`
    Status QraiopStatus `json:"status,omitempty"`
}

//+kubebuilder:object:root=true

// QraiopList contains a list of Qraiop
type QraiopList struct {
    metav1.TypeMeta `json:",inline"`
    metav1.ListMeta `json:"metadata,omitempty"`
    Items           []Qraiop `json:"items"`
}

func init() {
    SchemeBuilder.Register(&Qraiop{}, &QraiopList{})
}
