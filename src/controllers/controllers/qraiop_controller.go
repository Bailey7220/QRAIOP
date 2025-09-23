// src/controllers/controllers/qraiop_controller.go
package controllers

import (
	"context"
	"fmt"
	"time"

	"github.com/go-logr/logr"
	appsv1 "k8s.io/api/apps/v1"
	corev1 "k8s.io/api/core/v1"
	networkingv1 "k8s.io/api/networking/v1"
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/apimachinery/pkg/api/resource"
	"k8s.io/apimachinery/pkg/runtime"
	"k8s.io/apimachinery/pkg/util/intstr"
	ctrl "sigs.k8s.io/controller-runtime"
	"sigs.k8s.io/controller-runtime/pkg/client"
	"sigs.k8s.io/controller-runtime/pkg/controller/controllerutil"

	qraiopv1 "github.com/Bailey7220/QRAIOP/controllers/api/v1"
)

// QraiopReconciler reconciles a Qraiop object
type QraiopReconciler struct {
	client.Client
	Scheme *runtime.Scheme
	Log    logr.Logger
}

//+kubebuilder:rbac:groups=qraiop.io,resources=qraiops,verbs=get;list;watch;create;update;patch;delete
//+kubebuilder:rbac:groups=qraiop.io,resources=qraiops/status,verbs=get;update;patch
//+kubebuilder:rbac:groups=qraiop.io,resources=qraiops/finalizers,verbs=update
//+kubebuilder:rbac:groups=apps,resources=deployments,verbs=get;list;watch;create;update;patch;delete
//+kubebuilder:rbac:groups=core,resources=services;configmaps;secrets;serviceaccounts,verbs=get;list;watch;create;update;patch;delete
//+kubebuilder:rbac:groups=networking.k8s.io,resources=networkpolicies,verbs=get;list;watch;create;update;patch;delete

// Reconcile is part of the main kubernetes reconciliation loop
func (r *QraiopReconciler) Reconcile(ctx context.Context, req ctrl.Request) (ctrl.Result, error) {
	log := r.Log.WithValues("qraiop", req.NamespacedName)

	// Fetch the Qraiop instance
	var qraiop qraiopv1.Qraiop
	if err := r.Get(ctx, req.NamespacedName, &qraiop); err != nil {
		log.Error(err, "unable to fetch Qraiop")
		return ctrl.Result{}, client.IgnoreNotFound(err)
	}

	// Initialize status if not set
	if qraiop.Status.Phase == "" {
		qraiop.Status.Phase = "Initializing"
		qraiop.Status.Components = make(map[string]qraiopv1.ComponentStatus)
		r.updateStatus(ctx, &qraiop, "Initializing QRAIOP components")
	}

	// Reconcile components based on spec
	if err := r.reconcileComponents(ctx, &qraiop); err != nil {
		log.Error(err, "failed to reconcile components")
		r.updateStatus(ctx, &qraiop, fmt.Sprintf("Error: %v", err))
		return ctrl.Result{RequeueAfter: time.Minute}, err
	}

	// Update final status
	r.updateStatus(ctx, &qraiop, "All components ready")
	qraiop.Status.Phase = "Ready"

	return ctrl.Result{RequeueAfter: time.Minute * 10}, nil
}

func (r *QraiopReconciler) reconcileComponents(ctx context.Context, qraiop *qraiopv1.Qraiop) error {
	// Reconcile cryptography component
	if err := r.reconcileCryptography(ctx, qraiop); err != nil {
		return fmt.Errorf("failed to reconcile cryptography: %w", err)
	}
	return nil
}

func (r *QraiopReconciler) reconcileCryptography(ctx context.Context, qraiop *qraiopv1.Qraiop) error {
	if !qraiop.Spec.Cryptography.Enabled {
		r.setComponentStatus(qraiop, "cryptography", "Disabled", "Cryptography component is disabled")
		return nil
	}

	// Create crypto service deployment
	deployment := &appsv1.Deployment{
		ObjectMeta: metav1.ObjectMeta{
			Name:      "qraiop-crypto",
			Namespace: qraiop.Namespace,
		},
		Spec: appsv1.DeploymentSpec{
			Replicas: int32Ptr(1),
			Selector: &metav1.LabelSelector{
				MatchLabels: map[string]string{
					"app": "qraiop-crypto",
				},
			},
			Template: corev1.PodTemplateSpec{
				ObjectMeta: metav1.ObjectMeta{
					Labels: map[string]string{
						"app": "qraiop-crypto",
					},
				},
				Spec: corev1.PodSpec{
					Containers: []corev1.Container{
						{
							Name:  "crypto-service",
							Image: "ghcr.io/bailey7220/qraiop-crypto:latest",
							Ports: []corev1.ContainerPort{
								{
									ContainerPort: 8080,
									Name:          "http",
								},
							},
							Resources: corev1.ResourceRequirements{
								Limits: corev1.ResourceList{
									"cpu":    resource.MustParse("500m"),
									"memory": resource.MustParse("512Mi"),
								},
								Requests: corev1.ResourceList{
									"cpu":    resource.MustParse("100m"),
									"memory": resource.MustParse("128Mi"),
								},
							},
						},
					},
				},
			},
		},
	}

	// Set controller reference
	if err := controllerutil.SetControllerReference(qraiop, deployment, r.Scheme); err != nil {
		return err
	}

	// Create or update deployment
	if err := r.createOrUpdateDeployment(ctx, deployment); err != nil {
		r.setComponentStatus(qraiop, "cryptography", "Error", err.Error())
		return err
	}

	r.setComponentStatus(qraiop, "cryptography", "Ready", "Cryptography service is running")
	return nil
}

// Helper functions
func (r *QraiopReconciler) createOrUpdateDeployment(ctx context.Context, deployment *appsv1.Deployment) error {
	found := &appsv1.Deployment{}
	err := r.Get(ctx, client.ObjectKeyFromObject(deployment), found)

	if err != nil && client.IgnoreNotFound(err) != nil {
		return err
	}

	if err != nil {
		return r.Create(ctx, deployment)
	} else {
		deployment.ResourceVersion = found.ResourceVersion
		return r.Update(ctx, deployment)
	}
}

func (r *QraiopReconciler) setComponentStatus(qraiop *qraiopv1.Qraiop, component, status, message string) {
	if qraiop.Status.Components == nil {
		qraiop.Status.Components = make(map[string]qraiopv1.ComponentStatus)
	}

	qraiop.Status.Components[component] = qraiopv1.ComponentStatus{
		Status:      status,
		Message:     message,
		LastUpdated: metav1.Now(),
	}
}

func (r *QraiopReconciler) updateStatus(ctx context.Context, qraiop *qraiopv1.Qraiop, message string) error {
	qraiop.Status.Message = message
	qraiop.Status.LastUpdated = metav1.Now()
	return r.Status().Update(ctx, qraiop)
}

func int32Ptr(i int32) *int32 {
	return &i
}

// SetupWithManager sets up the controller with the Manager.
func (r *QraiopReconciler) SetupWithManager(mgr ctrl.Manager) error {
	return ctrl.NewControllerManagedBy(mgr).
		For(&qraiopv1.Qraiop{}).
		Owns(&appsv1.Deployment{}).
		Complete(r)
}
