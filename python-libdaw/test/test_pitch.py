import unittest
from libdaw.pitch import ScientificPitch, A440, Pitch, PitchClass, PitchName

class TestPitch(unittest.TestCase):
    def test_reference_pitches(self):
        self.assertAlmostEqual(
            A440().resolve(Pitch(octave=4, pitch_class=PitchClass(name=PitchName.A))),
            440,
        )
        self.assertAlmostEqual(
            ScientificPitch().resolve(Pitch(octave=4, pitch_class=PitchClass(name=PitchName.C))),
            256,
        )
