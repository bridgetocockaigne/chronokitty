Feature: Store a task

    Scenario: Tracker stores a task
        Given a task with name "kitty works"
        And a storage
        When the tracker stores a task in storage
        Then the task gets an id
        And the task can be found in storage


