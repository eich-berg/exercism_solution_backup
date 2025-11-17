from datetime import time, timedelta

class Clock:
    def __init__(self, hour, minute):
        # Convert everything to minutes and handle overflow
        total_minutes = hour * 60 + minute
        # Calculate hours and minutes within 24-hour cycle
        self.hours_total = (total_minutes // 60) % 24
        self.minutes_total = total_minutes % 60
        # Ensure positive modulo results
        if self.hours_total < 0:
            self.hours_total += 24
        if self.minutes_total < 0:
            self.minutes_total += 60
            self.hours_total = (self.hours_total - 1) % 24

    def __repr__(self):
        return f"Clock({self.hours_total}, {self.minutes_total})"

    def __str__(self):
        return f"{self.hours_total:02d}:{self.minutes_total:02d}"

    def __eq__(self, other):
        return self.hours_total == other.hours_total and self.minutes_total == other.minutes_total

    def __add__(self, minutes):
        return Clock(self.hours_total, self.minutes_total + minutes)

    def __sub__(self, minutes):
        return Clock(self.hours_total, self.minutes_total - minutes)