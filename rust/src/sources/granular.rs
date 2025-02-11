use std::sync::Arc;
use crate::dsp::buffer::Buffer;

struct Grain {
    pub start: usize,
    pub length: usize,
    pub order: usize,
    pub buffer: Arc<Buffer>,

    position: usize,
    cleanup_countdown: usize,
    active: bool,
    schedule_cleanup: bool,
}

impl Grain {
    pub fn new(order: usize, length: usize, start: usize, buffer: Arc<Buffer>) -> Self {
        let mut s = Self {
            start,
            length,
            order,
            buffer,

            position: 0,
            cleanup_countdown: length,
            active: false,
            schedule_cleanup: false,
        };

        s.wrapLength();

        s
    }

    pub fn process(&mut self, env_length: f32, env_shape: usize, spatialize: f32) {
        if (self.checkPosition()) {
            let mut tmp: f32 = self.buffer.get_current_sample();

            self.apply_windowing(&mut tmp, env_length, env_shape);

            if (spatialize > 0.0 && spatializer < spatialize) {
                // First, randomly select a channel

                if (channelizer > 0.5) {
                    // Pan to the right
                    sample[0] = tmp * ((1.0 - spatializer) / 2);
                    sample[1] = tmp * ((1.0 + spatializer) / 2);
                } else {
                    // Pan to the left
                    sample[0] = tmp * ((1.0 + spatializer) / 2);
                    sample[1] = tmp * ((1.0 - spatializer) / 2);
                }
            } else {
                sample[0] = tmp * 0.5;
                sample[1] = tmp * 0.5;
            }

            self.position += 1;
        } else {
            sample[0] = 0.0;
            sample[1] = 0.0;
        }

        return sample;
    }

    pub fn check() -> usize {
        if (!_active && !_schedule_cleanup) {
            // Grain isn't active or scheduled for deletion, check position
            if (checkPosition()) {
                _active = true;
                position = 0;

                // Set spatialization values
                spatializer = (float)
                random();
                channelizer = (float)
                random();

                return 1;
            }
        } else if (!_active && _schedule_cleanup) {
            // Make sure that the grain can't be deleted while still part of the cycle
            if (_cleanup_countdown > 0) {
                _cleanup_countdown -= 1;
                return 0;
            } else {
                // Grain is scheduled for deletion
                return 2;
            }
        } else if (_active && !checkPosition()) {
            // Grain is no longer active
            deactivate();
        } else if (_active) {
            // Grain is active
            return 3;
        }
        return 0;
    }

    float Grain::applyWindowing(float sample, float env_length, int env_shape) const {
    int env_size = (int) env_length * length;

    if (env_shape == 0) {
    // Handle a linear slope
    if (position < = env_size) {
    // Rise
    sample = sample * position * (1.0 / env_size);
    } else if (position > = length - env_size) {
    // Fall
    sample = sample * (1.0 - (position - (length - env_size)) * (1.0 / env_size));
    }
    } else {
// TODO: implement exponential slopes
// Handle exponential
//       if(position < env_size) {
//         sample = sample * env_step * pow(position, 2);
//       } else {
//         sample = sample * env_step * pow(position * -1, 2) + 1;
//       }
    }

    return sample;
}

void Grain::deactivate() {
_active = false;
}

void Grain::scheduleCleanup() {
_schedule_cleanup = true;
}

void Grain::loop() {
_loop = ! _loop;
}

void Grain::setLength(int nLength) {
length = nLength;
_cleanup_countdown = length;

wrapLength();
}

void Grain::calculateStart(int division) {
start = order * division;
wrapLength();
}

void Grain::wrapLength() {
end = start + length;

if (end > buffer -> getSize()) {
end -= buffer -> getSize();
}
}

bool Grain::checkPosition() {
// Check if the buffer crosses 0
if (start > end) {
return buffer -> getPosition() > start | | buffer -> getPosition() < end;
} else {
return buffer -> getPosition() > start & & buffer -> getPosition() < end;
}
}

int Grain::getOrder() {
return order;
}

int Grain::getStart() {
return start;
}