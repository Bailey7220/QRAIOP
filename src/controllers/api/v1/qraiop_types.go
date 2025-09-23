// src/controllers/api/v1/qraiop_types.go
package v1

import (
    metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
    "k8s.io/apimachinery/pkg/runtime"
)

// ComponentStatus defines individual component status
type ComponentStatus struct {
    Status      string      `json:"status"`
    Message     string      `json:"message,omitempty"`
    LastUpdated metav1.Time `json:"lastUpdated,omitempty"`
}

// QraiopSpec defines the desired state of Qraiop
type QraiopSpec struct {
    // Add your spec fields here
}

// QraiopStatus defines the observed state of Qraiop
type QraiopStatus struct {
    Phase       string                     `json:"phase,omitempty"`
    Message     string                     `json:"message,omitempty"`
    Components  map[string]ComponentStatus `json:"components,omitempty"`
    LastUpdated metav1.Time                `json:"lastUpdated,omitempty"`
    Conditions  []metav1.Condition         `json:"conditions,omitempty"`
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
