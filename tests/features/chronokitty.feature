Feature: Track a task time

    Scenario: Tracker starts a task
        Given a task with name "kitty works"
        When the tracker starts the task
        Then the task is started

    Scenario: Tracker stops a task
        Given a task with name "kitty works"
        When the tracker stops the task
        Then the task is stoped

    Scenario: Tracker starts a task
        Given a task with name "kitty works"
        When the tracker starts the task
        And the tracker stops the task
        Then the task have a duration bigger then zero

    Scenario: Tracker starts a task
        Given a task with name "kitty works"
        When the tracker stops the task
        Then the task have a duration is zero

    Scenario: Tracker starts a task
        Given a task with name "kitty works"
        When the tracker starts the task
        Then the task have a duration is zero

