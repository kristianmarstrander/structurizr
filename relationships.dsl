// People - systems
student -> vendorPortal "Accesses learning"
supervisor -> vendorDebrief "Runs debrief"
student -> webApp "Accesses App"

// Vendor - ExampleCorp handoff
vendorPortal -> sessionManager "session id & debrief service auth"
vendorDebrief -> vendorResults "Record/view results"
webApp -> vendorResults "Push results for supervisor review"

// vLaunch & WebApp
sessionManager -> webApp "Start/return to session"

// WebApp - Simulation flow
webApp -> analyticsEngine "pre-sim data/score"
webApp -> simulation "in-sim actions & results (UI updates)"

// Orchestration/state
simulation -> stateSvc "user actions & session state"
stateSvc   -> stt "speech inputs (ASR)"
simulation -> caiQ "question prompts"
simulation -> caiEdu "educational responses"
simulation -> tts "speech output synthesis"

// In-sim model decisions
simulation -> processingEngine "invoke model for in-sim decisions"

// Results & telemetry
simulation -> dataCapture "user actions & sim state"
dataCapture -> monitoringService "operational telemetry"
dataCapture -> analyticsService "analytics pipeline"

// Notes/implicit flows from diagram
stateSvc -> webApp "session id & PIN"
