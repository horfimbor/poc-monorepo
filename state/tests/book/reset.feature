Feature: Mono example

  Scenario: i can empty a new Mono
   Given a mono
    When i try to reset it
    Then it got a value of 0

  Scenario: i cant empty twice Mono
    Given a mono
    When i try to reset it
    When i try to reset it
    Then it got an error
