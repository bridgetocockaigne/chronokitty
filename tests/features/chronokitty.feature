Feature: Track a task time

    Scenario: Tracker starts a task
        When the tracker starts a new task with name "kitty works"
        Then the tracked time is greater then 0
