// src/controllers/api/v1/qraiop_types.go
package v1

import (
    metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
    "k8s.io/apimachinery/pkg/runtime"
)

// CryptographyConfig defines quantum-safe crypto settings
type CryptographyConfig struct {
    Enabled               bool                   `json:"enabled"`
    Algorithms            []string               `json:"algorithms,omitempty"`
    SecurityLevel         int                    `json:"securityLevel,omitempty"`
    HybridMode            bool                   `json:"hybridMode,omitempty"`
    CertificateManagement CertManagementConfig   `json:"certificateManagement,omitempty"`
}

// CertManagementConfig defines certificate management
type CertManagementConfig struct {
    AutoRotation         bool   `json:"autoRotation,omitempty"`
    RotationInterval     int    `json:"rotationInterval,omitempty"`
    CertificateAuthority string `json:"certificateAuthority,omitempty"`
}

// AIConfig defines AI orchestration settings
type AIConfig struct {
    Enabled     bool          `json:"enabled"`
    LLMProvider string        `json:"llmProvider,omitempty"`
    ModelConfig ModelConfig   `json:"modelConfig,omitempty"`
    Agents      []AgentConfig `json:"agents,omitempty"`
}

// ModelConfig defines LLM model settings
type ModelConfig struct {
    Model       string  `json:"model,omitempty"`
    Temperature float32 `json:"temperature,omitempty"`
    MaxTokens   int     `json:"maxTokens,omitempty"`
}

// AgentConfig defines individual agent settings
type AgentConfig struct {
    Type    string            `json:"type"`
    Enabled bool              `json:"enabled"`
    Config  map[string]string `json:"config,omitempty"`
}

// ChaosConfig defines chaos engineering settings
type ChaosConfig struct {
    Enabled   bool              `json:"enabled"`
    Schedules []ChaosSchedule   `json:"schedules,omitempty"`
    Safety    ChaosSafetyConfig `json:"safety,omitempty"`
}

// ChaosSchedule defines scheduled chaos experiments
type ChaosSchedule struct {
    Name             string                 `json:"name"`
    Schedule         string                 `json:"schedule"`
    ExperimentConfig map[string]interface{} `json:"experimentConfig"`
}

// ChaosSafetyConfig defines chaos safety settings
type ChaosSafetyConfig struct {
    MaxConcurrentExperiments int      `json:"maxConcurrentExperiments,omitempty"`
    ExcludedNamespaces       []string `json:"excludedNamespaces,omitempty"`
    BusinessHoursOnly        bool     `json:"businessHoursOnly,omitempty"`
}

// MonitoringConfig defines monitoring settings
type MonitoringConfig struct {
    Enabled    bool             `json:"enabled"`
    Prometheus PrometheusConfig `json:"prometheus,omitempty"`
    Grafana    GrafanaConfig    `json:"grafana,omitempty"`
    Alerting   AlertingConfig   `json:"alerting,omitempty"`
}

// PrometheusConfig defines Prometheus settings
type PrometheusConfig struct {
    Enabled        bool   `json:"enabled"`
    ScrapeInterval string `json:"scrapeInterval,omitempty"`
    Retention      string `json:"retention,omitempty"`
}

// GrafanaConfig defines Grafana settings
type GrafanaConfig struct {
    Enabled               bool `json:"enabled"`
    DashboardProvisioning bool `json:"dashboardProvisioning,omitempty"`
}

// AlertingConfig defines alerting settings
type AlertingConfig struct {
    Enabled  bool           `json:"enabled"`
    Channels []AlertChannel `json:"channels,omitempty"`
}

// AlertChannel defines alert channel configuration
type AlertChannel struct {
    Type   string            `json:"type"`
    Config map[string]string `json:"config"`
}

// SecurityConfig defines security policy settings
type SecurityConfig struct {
    NetworkPolicies      NetworkPolicyConfig `json:"networkPolicies,omitempty"`
    PodSecurityStandards PodSecurityConfig   `json:"podSecurityStandards,omitempty"`
    RBAC                 RBACConfig          `json:"rbac,omitempty"`
}

// NetworkPolicyConfig defines network policy settings
type NetworkPolicyConfig struct {
    DefaultDenyAll           bool `json:"defaultDenyAll,omitempty"`
    AllowQraiopCommunication bool `json:"allowQraiopCommunication,omitempty"`
}

// PodSecurityConfig defines pod security settings
type PodSecurityConfig struct {
    Level   string `json:"level,omitempty"`
    Enforce bool   `json:"enforce,omitempty"`
}

// RBACConfig defines RBAC settings
type RBACConfig struct {
    Enabled         bool                   `json:"enabled"`
    ServiceAccounts []ServiceAccountConfig `json:"serviceAccounts,omitempty"`
}

// ServiceAccountConfig defines service account settings
type ServiceAccountConfig struct {
    Name         string   `json:"name"`
    Namespace    string   `json:"namespace"`
    Roles        []string `json:"roles,omitempty"`
    ClusterRoles []string `json:"clusterRoles,omitempty"`
}

// QraiopSpec defines the desired state of Qraiop
type QraiopSpec struct {
    Cryptography     CryptographyConfig `json:"cryptography,omitempty"`
    AIOrchestration  AIConfig           `json:"aiOrchestration,omitempty"`
    ChaosEngineering ChaosConfig        `json:"chaosEngineering,omitempty"`
    Monitoring       MonitoringConfig   `json:"monitoring,omitempty"`
    SecurityPolicies SecurityConfig     `json:"securityPolicies,omitempty"`
}

// ComponentStatus defines individual component status
type ComponentStatus struct {
    Status      string      `json:"status"`
    Message     string      `json:"message,omitempty"`
    LastUpdated metav1.Time `json:"lastUpdated,omitempty"`
}

// QraiopStatus defines the observed state of Qraiop
type QraiopStatus struct {
    Phase       string                    `json:"phase,omitempty"`
    Message     string                    `json:"message,omitempty"`
    Components  map[string]ComponentStatus `json:"components,omitempty"`
    LastUpdated metav1.Time               `json:"lastUpdated,omitempty"`
    Conditions  []metav1.Condition        `json:"conditions,omitempty"`
}

// +kubebuilder:object:root=true
// +kubebuilder:subresource:status
// +kubebuilder:resource:scope=Namespaced,shortName=qraiop
// +kubebuilder:printcolumn:name="Phase",type=string,JSONPath=".status.phase"
// +kubebuilder:printcolumn:name="Age",type=date,JSONPath=".metadata.creationTimestamp"
type Qraiop struct {
    metav1.TypeMeta   `json:",inline"`
    metav1.ObjectMeta `json:"metadata,omitempty"`

    Spec   QraiopSpec   `json:"spec,omitempty"`
    Status QraiopStatus `json:"status,omitempty"`
}

// +kubebuilder:object:root=true
type QraiopList struct {
    metav1.TypeMeta `json:",inline"`
    metav1.ListMeta `json:"metadata,omitempty"`
    Items           []Qraiop `json:"items"`
}

// DeepCopyObject implements runtime.Object for Qraiop
func (in *Qraiop) DeepCopyObject() runtime.Object {
    if c := in.DeepCopy(); c != nil {
        return c
    }
    return nil
}

// DeepCopyObject implements runtime.Object for QraiopList
func (in *QraiopList) DeepCopyObject() runtime.Object {
    if c := in.DeepCopy(); c != nil {
        return c
    }
    return nil
}

func init() {
    SchemeBuilder.Register(&Qraiop{}, &QraiopList{})
}
