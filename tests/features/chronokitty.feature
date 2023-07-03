Feature: Save a task

    Scenario: Start a task
        Given a task
        And a storage
        When the user starts the task
        Then the task is stored
