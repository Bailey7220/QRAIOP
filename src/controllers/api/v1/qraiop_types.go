// src/controllers/api/v1/qraiop_types.go
package v1

import (
    metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
    "k8s.io/apimachinery/pkg/runtime"
)

// ... [all type definitions as before] ...

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

// ONLY these two DeepCopyObject methods:

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
