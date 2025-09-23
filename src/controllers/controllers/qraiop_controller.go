// src/controllers/controllers/qraiop_controller.go
package controllers

import (
    "context"
    "time"

    "github.com/go-logr/logr"
    metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
    "k8s.io/apimachinery/pkg/runtime"
    ctrl "sigs.k8s.io/controller-runtime"
    "sigs.k8s.io/controller-runtime/pkg/client"

    qraiopv1 "github.com/Bailey7220/QRAIOP/controllers/api/v1"
)

type QraiopReconciler struct {
    client.Client
    Scheme *runtime.Scheme
    Log    logr.Logger
}

// +kubebuilder:rbac:groups=qraiop.io,resources=qraiops,verbs=get;list;watch;create;update;patch;delete
// +kubebuilder:rbac:groups=qraiop.io,resources=qraiops/status,verbs=get;update;patch
func (r *QraiopReconciler) Reconcile(ctx context.Context, req ctrl.Request) (ctrl.Result, error) {
    log := r.Log.WithValues("qraiop", req.NamespacedName)

    var qraiop qraiopv1.Qraiop
    if err := r.Get(ctx, req.NamespacedName, &qraiop); err != nil {
        log.Error(err, "unable to fetch Qraiop")
        return ctrl.Result{}, client.IgnoreNotFound(err)
    }

    if qraiop.Status.Phase == "" {
        qraiop.Status.Phase = "Initializing"
        qraiop.Status.Components = make(map[string]qraiopv1.ComponentStatus)
        qraiop.Status.LastUpdated = metav1.Now()
        _ = r.Status().Update(ctx, &qraiop)
    }

    // Example component readiness update
    qraiop.Status.Components["cryptography"] = qraiopv1.ComponentStatus{
        Status:      "Ready",
        Message:     "OK",
        LastUpdated: metav1.Now(),
    }

    qraiop.Status.Phase = "Ready"
    qraiop.Status.LastUpdated = metav1.Now()
    _ = r.Status().Update(ctx, &qraiop)

    return ctrl.Result{RequeueAfter: time.Minute * 10}, nil
}

func (r *QraiopReconciler) SetupWithManager(mgr ctrl.Manager) error {
    return ctrl.NewControllerManagedBy(mgr).
        For(&qraiopv1.Qraiop{}).
        Complete(r)
}
