systemLandscape "corePlatformLandscape" {
    include *
    exclude securityBaseline
    autoLayout
    description "The high-level system landscape showing all people and software systems."
}

systemContext corePlatform "corePlatformContext" {
    include *
    exclude securityBaseline
    autoLayout
    description "The system context view for corePlatform, showing its students and direct interactions with other systems."
}

container corePlatform "corePlatformContainers" {
    include *
    autoLayout
    description "The container view for corePlatform, showing its direct interactions."
}

filtered "corePlatformContainers" include "pii, Relationship" "corePlatformContainersWithPII"

dynamic corePlatform "AuthFlow" {
  autoLayout
  student -> vendorPortal "Authenticates"
  vendorPortal -> sessionManager "Issues Token"
  sessionManager -> webApp "Starts Session"
  student -> webApp "Accesses App"

}

dynamic corePlatform "AIFlow" {
  autoLayout
  simulation -> caiQ "Requests question"
  caiQ -> simulation "Returns question"
  simulation -> caiEdu "Requests explanation"
  caiEdu -> simulation "Returns explanation"
}

// ====== Styles ======
styles {
    element "Person" {
      shape person
      background #08427b
      color #ffffff
    }

    element "Software System" {
      background #1168bd
      color #ffffff
    }

    element "Container" {
      background #438dd5
      color #ffffff
    }

    element "Component" {
      background #85bbf0
      color #000000
    }

    element "Database" {
      shape cylinder
    }

    element "Web Browser" {
      shape webBrowser
    }

    element "Mobile" {
      shape mobileDevicePortrait
    }

    element "External System" {
      background #999999
      color #ffffff
    }

    element "confidential" {
      stroke #ff0000
      strokeWidth 30
    }

    element "restricted" {
      stroke #ffbf00
      strokeWidth 30
    }

    element "protected" {
      stroke #ffff00
      strokeWidth 30
    }

    element "public" {
      stroke #00ff00
      strokeWidth 30
    }
}
