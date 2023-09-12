from __future__ import annotations

import itertools

import rerun.experimental as rr2
from rerun.experimental import cmp as rr_cmp


def test_clear() -> None:
    clears: list[rr_cmp.ClearSettingsLike] = [
        # ClearLike: bool
        True,
        # ClearLike: Clear
        rr_cmp.ClearSettings(True),
    ]

    all_arrays = itertools.zip_longest(
        clears,
    )

    for clear in all_arrays:
        print(f"rr2.Clear(\n" f"settings={clear}\n" f")")
        arch = rr2.Clear(clear)
        print(f"{arch}\n")

        assert arch.settings == rr_cmp.ClearSettingsArray.from_similar([True])


if __name__ == "__main__":
    test_clear()
