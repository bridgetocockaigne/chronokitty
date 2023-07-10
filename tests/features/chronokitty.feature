Feature: Tracking a task

    Scenario: Tracking a task
        Given a task
        And a storage
        When the user starts the task
        And the user stops the task
        Then the task is stored
        And the task as a duration bigger then 0
