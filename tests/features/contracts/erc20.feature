Feature: ERC20
  Scenario: If we deploy ERC20 token it works
    Given ERC20 token is deployed
    Then total supply is 10000
    And symbol is PLS
    And name is Plascoin
    And decimals is 10