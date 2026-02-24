workspace "Example App" "A ourCompany product that perfect your shock pad skills." {

    // !identifiers hierarchical

    !docs docs
    !adrs adrs

    model {
        !include model.dsl
        !include relationships.dsl

        securityBaseline = softwareSystem "Security Baseline" "Provides security features and compliance." {
            tags "Security baseline"
            !docs docs/baseline
        }
    }

    views {
        !include views.dsl
    }
}
