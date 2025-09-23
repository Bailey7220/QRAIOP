// src/controllers/api/v1/qraiop_types.go
package v1

import (
    metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
)

// QraiopSpec defines the desired state of Qraiop
type QraiopSpec struct {
    Cryptography CryptographyConfig `json:"cryptography,omitempty"`
}

// CryptographyConfig defines quantum-safe crypto settings
type CryptographyConfig struct {
    Enabled bool `json:"enabled"`
}

// QraiopStatus defines the observed state of Qraiop
type QraiopStatus struct {
    Phase   string `json:"phase,omitempty"`
    Message string `json:"message,omitempty"`
}

// +kubebuilder:object:root=true
// +kubebuilder:subresource:status
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

func init() {
    SchemeBuilder.Register(&Qraiop{}, &QraiopList{})
}
