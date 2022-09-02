Feature: ERC20
  Scenario: If we deploy ERC20 token it works
    Given ERC20 token is deployed
    Then total supply is 10000
    And symbol is PLS
    And name is Plascoin
    And decimals is 10

  Scenario: If we transfer more than initial supply it throws error
    Given ERC20 token is deployed
    Then I transfer 10001 PLS to account 1 and it throws an error

  Scenario: Transfer works
    Given ERC20 token is deployed
    When I transfer 1000 PLS to account 1
    Then I have 9000 PLS
    And account 1 has 1000 PLS
    And Transfer event is emitted

  Scenario: Transfer and approval works
    Given ERC20 token is deployed
    When I approve 1000 PLS for account 1
    And account 1 transfers 1000 PLS from account 0 to account 1
    Then I have 9000 PLS
    And account 1 has 1000 PLS
    And Approval event is emitted at -2
    And Transfer event is emitted at -1
