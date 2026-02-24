// People
student = person "Student" "Primary user of the system"
supervisor = person "Supervisor" "Monitors and manages users and the system"


group "someVendor" {
    vendorPortal = softwareSystem "Auth Portal" "Portal for user authentication and access" {
        tags "confidential", "pii"
    }
    vendorDebrief = softwareSystem "Reporting Service" "Generates and displays reports" {
        tags "confidential", "pii"
    }
    vendorResults = softwareSystem "Data Repository" "Stores and manages system data" {
        tags "confidential", "pii"
    }
}


group "ourCompany" {
    corePlatform = softwareSystem "Core Platform" "The main application platform" {
        tags "restricted", "pii"
        sessionManager = container "Session Manager" "Manages user sessions and authentication" "Service" {
            tags "confidential", "pii"
        }
        webApp = container "WebApp" "User interface for the platform" "SPA" {
            tags "confidential", "pii"
        }
        simulation = container "Core Service" "The core service of the platform" "Service" {
            tags "protected"
        }
        stateSvc = container "State Manager" "Manages the state of the system" "Service" {
            tags "confidential", "pii"
        }
        processingEngine = container "Processing Engine" "Handles real-time data processing" "Service" {
            tags "restricted", "pii"
        }
        analyticsEngine = container "Analytics Engine" "Performs pre-processing and analytics" "Service" {
            tags "restricted", "pii"
        }
        dataCapture = container "Data Capture" "Captures user interactions and system events" "Service" {
            tags "restricted", "pii"
        }
    }
}

// --- Trust boundary: 3rd Party ---
group "3rd Party" {
    stt = softwareSystem "Speech to Text Service" "ASR service" {
        tags "External System, AI Service, protected"
    }

    tts = softwareSystem "Text to Speech Service" "Speech synthesis" {
        tags "External System, AI Service, protected"
    }

    caiQ = softwareSystem "Conversation AI - questions" "Conversational AI for questioning" {
        tags "External System, AI Service, protected"
    }

    caiEdu = softwareSystem "Conversation AI - educational" "Conversational AI for education/explanations" {
        tags "External System, AI Service, protected"
    }

    monitoringService = softwareSystem "Monitoring Service" "Monitoring/telemetry" {
        tags "External System, protected"
    }

    analyticsService = softwareSystem "Analytics Service" "Analytics/lakehouse" {
            tags "External System, public"
    }

}
