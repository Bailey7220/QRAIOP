// src/controllers/controllers/qraiop_controller.go
package controllers

import (
    "context"
    "fmt"
    "time"

    "github.com/go-logr/logr"
    appsv1 "k8s.io/api/apps/v1"
    corev1 "k8s.io/api/core/v1"
    rbacv1 "k8s.io/api/rbac/v1"
    networkingv1 "k8s.io/api/networking/v1"
    metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
    "k8s.io/apimachinery/pkg/api/resource"  // ADD THIS MISSING IMPORT
    "k8s.io/apimachinery/pkg/runtime"
    "k8s.io/apimachinery/pkg/util/intstr"
    ctrl "sigs.k8s.io/controller-runtime"
    "sigs.k8s.io/controller-runtime/pkg/client"
    "sigs.k8s.io/controller-runtime/pkg/controller/controllerutil"
    "sigs.k8s.io/controller-runtime/pkg/log"

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
//+kubebuilder:rbac:groups=rbac.authorization.k8s.io,resources=roles;rolebindings;clusterroles;clusterrolebindings,verbs=get;list;watch;create;update;patch;delete
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

    // Reconcile AI orchestration
    if err := r.reconcileAIOrchestration(ctx, qraiop); err != nil {
        return fmt.Errorf("failed to reconcile AI orchestration: %w", err)
    }

    // Reconcile chaos engineering
    if err := r.reconcileChaosEngineering(ctx, qraiop); err != nil {
        return fmt.Errorf("failed to reconcile chaos engineering: %w", err)
    }

    // Reconcile monitoring
    if err := r.reconcileMonitoring(ctx, qraiop); err != nil {
        return fmt.Errorf("failed to reconcile monitoring: %w", err)
    }

    // Reconcile security policies
    if err := r.reconcileSecurityPolicies(ctx, qraiop); err != nil {
        return fmt.Errorf("failed to reconcile security policies: %w", err)
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
            Replicas: int32Ptr(2),
            Selector: &metav1.LabelSelector{
                MatchLabels: map[string]string{
                    "app":       "qraiop-crypto",
                    "component": "cryptography",
                },
            },
            Template: corev1.PodTemplateSpec{
                ObjectMeta: metav1.ObjectMeta{
                    Labels: map[string]string{
                        "app":       "qraiop-crypto",
                        "component": "cryptography",
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
                            Env: []corev1.EnvVar{
                                {
                                    Name:  "SECURITY_LEVEL",
                                    Value: fmt.Sprintf("%d", qraiop.Spec.Cryptography.SecurityLevel),
                                },
                                {
                                    Name:  "HYBRID_MODE",
                                    Value: fmt.Sprintf("%t", qraiop.Spec.Cryptography.HybridMode),
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

    // Create service
    service := &corev1.Service{
        ObjectMeta: metav1.ObjectMeta{
            Name:      "qraiop-crypto",
            Namespace: qraiop.Namespace,
        },
        Spec: corev1.ServiceSpec{
            Selector: map[string]string{
                "app": "qraiop-crypto",
            },
            Ports: []corev1.ServicePort{
                {
                    Port:       80,
                    TargetPort: intstr.FromInt(8080),
                    Name:       "http",
                },
            },
        },
    }

    if err := controllerutil.SetControllerReference(qraiop, service, r.Scheme); err != nil {
        return err
    }

    if err := r.createOrUpdateService(ctx, service); err != nil {
        return err
    }

    r.setComponentStatus(qraiop, "cryptography", "Ready", "Cryptography service is running")
    return nil
}

func (r *QraiopReconciler) reconcileAIOrchestration(ctx context.Context, qraiop *qraiopv1.Qraiop) error {
    if !qraiop.Spec.AIOrchestration.Enabled {
        r.setComponentStatus(qraiop, "ai-orchestration", "Disabled", "AI orchestration is disabled")
        return nil
    }

    // Create AI orchestration deployment
    deployment := &appsv1.Deployment{
        ObjectMeta: metav1.ObjectMeta{
            Name:      "qraiop-ai",
            Namespace: qraiop.Namespace,
        },
        Spec: appsv1.DeploymentSpec{
            Replicas: int32Ptr(1),
            Selector: &metav1.LabelSelector{
                MatchLabels: map[string]string{
                    "app":       "qraiop-ai",
                    "component": "ai-orchestration",
                },
            },
            Template: corev1.PodTemplateSpec{
                ObjectMeta: metav1.ObjectMeta{
                    Labels: map[string]string{
                        "app":       "qraiop-ai",
                        "component": "ai-orchestration",
                    },
                },
                Spec: corev1.PodSpec{
                    Containers: []corev1.Container{
                        {
                            Name:  "ai-orchestration",
                            Image: "ghcr.io/bailey7220/qraiop-ai:latest",
                            Ports: []corev1.ContainerPort{
                                {
                                    ContainerPort: 8080,
                                    Name:          "http",
                                },
                            },
                            Env: []corev1.EnvVar{
                                {
                                    Name:  "LLM_PROVIDER",
                                    Value: qraiop.Spec.AIOrchestration.LLMProvider,
                                },
                                {
                                    Name:  "MODEL_NAME",
                                    Value: qraiop.Spec.AIOrchestration.ModelConfig.Model,
                                },
                            },
                            Resources: corev1.ResourceRequirements{
                                Limits: corev1.ResourceList{
                                    "cpu":    resource.MustParse("1000m"),
                                    "memory": resource.MustParse("1Gi"),
                                },
                                Requests: corev1.ResourceList{
                                    "cpu":    resource.MustParse("200m"),
                                    "memory": resource.MustParse("256Mi"),
                                },
                            },
                        },
                    },
                },
            },
        },
    }

    if err := controllerutil.SetControllerReference(qraiop, deployment, r.Scheme); err != nil {
        return err
    }

    if err := r.createOrUpdateDeployment(ctx, deployment); err != nil {
        r.setComponentStatus(qraiop, "ai-orchestration", "Error", err.Error())
        return err
    }

    r.setComponentStatus(qraiop, "ai-orchestration", "Ready", "AI orchestration is running")
    return nil
}

func (r *QraiopReconciler) reconcileChaosEngineering(ctx context.Context, qraiop *qraiopv1.Qraiop) error {
    if !qraiop.Spec.ChaosEngineering.Enabled {
        r.setComponentStatus(qraiop, "chaos-engineering", "Disabled", "Chaos engineering is disabled")
        return nil
    }

    // Create chaos engineering deployment with appropriate RBAC
    deployment := &appsv1.Deployment{
        ObjectMeta: metav1.ObjectMeta{
            Name:      "qraiop-chaos",
            Namespace: qraiop.Namespace,
        },
        Spec: appsv1.DeploymentSpec{
            Replicas: int32Ptr(1),
            Selector: &metav1.LabelSelector{
                MatchLabels: map[string]string{
                    "app":       "qraiop-chaos",
                    "component": "chaos-engineering",
                },
            },
            Template: corev1.PodTemplateSpec{
                ObjectMeta: metav1.ObjectMeta{
                    Labels: map[string]string{
                        "app":       "qraiop-chaos",
                        "component": "chaos-engineering",
                    },
                },
                Spec: corev1.PodSpec{
                    ServiceAccountName: "qraiop-chaos",
                    Containers: []corev1.Container{
                        {
                            Name:  "chaos-engineering",
                            Image: "ghcr.io/bailey7220/qraiop-chaos:latest",
                            Env: []corev1.EnvVar{
                                {
                                    Name:  "MAX_CONCURRENT_EXPERIMENTS",
                                    Value: fmt.Sprintf("%d", qraiop.Spec.ChaosEngineering.Safety.MaxConcurrentExperiments),
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

    if err := controllerutil.SetControllerReference(qraiop, deployment, r.Scheme); err != nil {
        return err
    }

    if err := r.createOrUpdateDeployment(ctx, deployment); err != nil {
        r.setComponentStatus(qraiop, "chaos-engineering", "Error", err.Error())
        return err
    }

    r.setComponentStatus(qraiop, "chaos-engineering", "Ready", "Chaos engineering is running")
    return nil
}

func (r *QraiopReconciler) reconcileMonitoring(ctx context.Context, qraiop *qraiopv1.Qraiop) error {
    if !qraiop.Spec.Monitoring.Enabled {
        r.setComponentStatus(qraiop, "monitoring", "Disabled", "Monitoring is disabled")
        return nil
    }

    r.setComponentStatus(qraiop, "monitoring", "Ready", "Monitoring is configured")
    return nil
}

func (r *QraiopReconciler) reconcileSecurityPolicies(ctx context.Context, qraiop *qraiopv1.Qraiop) error {
    // Create network policies if enabled
    if qraiop.Spec.SecurityPolicies.NetworkPolicies.DefaultDenyAll {
        networkPolicy := &networkingv1.NetworkPolicy{
            ObjectMeta: metav1.ObjectMeta{
                Name:      "qraiop-default-deny",
                Namespace: qraiop.Namespace,
            },
            Spec: networkingv1.NetworkPolicySpec{
                PodSelector: metav1.LabelSelector{},
                PolicyTypes: []networkingv1.PolicyType{
                    networkingv1.PolicyTypeIngress,
                    networkingv1.PolicyTypeEgress,
                },
            },
        }

        if err := controllerutil.SetControllerReference(qraiop, networkPolicy, r.Scheme); err != nil {
            return err
        }

        if err := r.createOrUpdateNetworkPolicy(ctx, networkPolicy); err != nil {
            return err
        }
    }

    r.setComponentStatus(qraiop, "security-policies", "Ready", "Security policies applied")
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
        // Create deployment
        return r.Create(ctx, deployment)
    } else {
        // Update deployment
        deployment.ResourceVersion = found.ResourceVersion
        return r.Update(ctx, deployment)
    }
}

func (r *QraiopReconciler) createOrUpdateService(ctx context.Context, service *corev1.Service) error {
    found := &corev1.Service{}
    err := r.Get(ctx, client.ObjectKeyFromObject(service), found)

    if err != nil && client.IgnoreNotFound(err) != nil {
        return err
    }

    if err != nil {
        return r.Create(ctx, service)
    } else {
        service.ResourceVersion = found.ResourceVersion
        service.Spec.ClusterIP = found.Spec.ClusterIP
        return r.Update(ctx, service)
    }
}

func (r *QraiopReconciler) createOrUpdateNetworkPolicy(ctx context.Context, np *networkingv1.NetworkPolicy) error {
    found := &networkingv1.NetworkPolicy{}
    err := r.Get(ctx, client.ObjectKeyFromObject(np), found)

    if err != nil && client.IgnoreNotFound(err) != nil {
        return err
    }

    if err != nil {
        return r.Create(ctx, np)
    } else {
        np.ResourceVersion = found.ResourceVersion
        return r.Update(ctx, np)
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
        Owns(&corev1.Service{}).
        Owns(&networkingv1.NetworkPolicy{}).
        Complete(r)
}
