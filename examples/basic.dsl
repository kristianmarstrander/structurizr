workspace "Big Bank plc" "Example workspace" {

    model {
        customer = person "Personal Banking Customer" "A customer of the bank." {
            tags "External"
        }
        
        enterprise "Big Bank plc" {
            supportStaff = person "Customer Service Staff" "Support staff within the bank."
            
            mainframe = softwareSystem "Mainframe Banking System" "Stores all core banking information." {
                tags "Legacy"
            }
            
            internetBankingSystem = softwareSystem "Internet Banking System" "Allows customers to view account info and make payments." {
                webApp = container "Web Application" "Delivers static content and the single page app." "Java and Spring MVC" {
                    tags "WebApp"
                }
                
                singlePageApp = container "Single-Page App" "Provides banking functionality via browser." "JavaScript and Angular" {
                    tags "Browser"
                }
                
                apiApplication = container "API Application" "Provides banking functionality via API." "Java and Spring Boot" {
                    signinController = component "Sign In Controller" "Allows users to sign in." "Spring MVC Controller"
                    accountsController = component "Accounts Controller" "Provides account information." "Spring MVC Controller"
                    securityComponent = component "Security Component" "Handles authentication." "Spring Security"
                }
                
                database = container "Database" "Stores user info and accounts." "PostgreSQL" {
                    tags "Database"
                }
            }
        }
        
        # Relationships
        customer -> internetBankingSystem "Views account and makes payments"
        customer -> supportStaff "Asks questions to" "Telephone"
        supportStaff -> mainframe "Uses"
        internetBankingSystem -> mainframe "Gets account info from"
        
        webApp -> singlePageApp "Delivers"
        singlePageApp -> apiApplication "Makes API calls to" "JSON/HTTPS"
        apiApplication -> database "Reads from and writes to" "SQL/TCP"
        apiApplication -> mainframe "Gets account info from" "XML/HTTPS"
    }

    views {
        systemLandscape "SystemLandscape" "Overview of the system landscape" {
            include *
            autolayout tb
        }
        
        systemContext internetBankingSystem "SystemContext" "System context for Internet Banking" {
            include *
            autolayout tb
        }
        
        container internetBankingSystem "Containers" "Container view" {
            include *
            autolayout lr
        }
        
        component apiApplication "Components" "Component view of API Application" {
            include *
            autolayout tb
        }

        styles {
            element "Element" {
                shape roundedbox
                background "#438dd5"
                color "#ffffff"
                fontSize 24
            }
            element "Person" {
                shape person
                background "#08427b"
            }
            element "External" {
                background "#999999"
            }
            element "Database" {
                shape cylinder
            }
            element "Browser" {
                shape webbrowser
            }
            element "WebApp" {
                shape webbrowser
            }
            element "Legacy" {
                background "#999999"
                shape box
            }
            relationship "Relationship" {
                color "#707070"
                thickness 2
            }
        }
    }
}
