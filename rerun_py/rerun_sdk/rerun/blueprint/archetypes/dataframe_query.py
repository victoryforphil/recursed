# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/dataframe_query.fbs".

# You can extend this class by creating a "DataframeQueryExt" class in "dataframe_query_ext.py".

from __future__ import annotations

from attrs import define, field

from ..._baseclasses import (
    Archetype,
)
from ...blueprint import components as blueprint_components
from .dataframe_query_ext import DataframeQueryExt

__all__ = ["DataframeQuery"]


@define(str=False, repr=False, init=False)
class DataframeQuery(DataframeQueryExt, Archetype):
    """**Archetype**: The query for the dataframe view."""

    # __init__ can be found in dataframe_query_ext.py

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            timeline=None,  # type: ignore[arg-type]
            filter_by_range=None,  # type: ignore[arg-type]
            filter_is_not_null=None,  # type: ignore[arg-type]
            apply_latest_at=None,  # type: ignore[arg-type]
            select=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> DataframeQuery:
        """Produce an empty DataframeQuery, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    timeline: blueprint_components.TimelineNameBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.TimelineNameBatch._optional,  # type: ignore[misc]
    )
    # The timeline for this query.
    #
    # If unset, the timeline currently active on the time panel is used.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    filter_by_range: blueprint_components.FilterByRangeBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.FilterByRangeBatch._optional,  # type: ignore[misc]
    )
    # If provided, only rows whose timestamp is within this range will be shown.
    #
    # Note: will be unset as soon as `timeline` is changed.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    filter_is_not_null: blueprint_components.FilterIsNotNullBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.FilterIsNotNullBatch._optional,  # type: ignore[misc]
    )
    # If provided, only show rows which contains a logged event for the specified component.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    apply_latest_at: blueprint_components.ApplyLatestAtBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.ApplyLatestAtBatch._optional,  # type: ignore[misc]
    )
    # Should empty cells be filled with latest-at queries?
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    select: blueprint_components.SelectedColumnsBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.SelectedColumnsBatch._optional,  # type: ignore[misc]
    )
    # Selected columns. If unset, all columns are selected.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]
