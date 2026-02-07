"""
RCSection Python API - v2

High-level API for generating reinforced concrete section drawings.
Outputs JSON compatible with the Typst/WASM plugin.
"""

import json
from dataclasses import dataclass, field
from typing import Optional, List, Tuple, Union


# --- Base Primitives ---

@dataclass
class Rect:
    x: float
    y: float
    width: float
    height: float
    rounded: Optional[float] = None

    def to_dict(self):
        d = {"type": "Rect", "x": self.x, "y": self.y, "width": self.width, "height": self.height}
        if self.rounded:
            d["rounded"] = self.rounded
        return d


@dataclass
class Circle:
    x: float
    y: float
    radius: float

    def to_dict(self):
        return {"type": "Circle", "x": self.x, "y": self.y, "radius": self.radius}


@dataclass
class Line:
    x1: float
    y1: float
    x2: float
    y2: float

    def to_dict(self):
        return {"type": "Line", "x1": self.x1, "y1": self.y1, "x2": self.x2, "y2": self.y2}


@dataclass
class Label:
    text: str
    x: float
    y: float
    mode: Optional[str] = None  # "callout" or "legend"

    def to_dict(self):
        d = {"type": "Label", "text": self.text, "x": self.x, "y": self.y}
        if self.mode:
            d["mode"] = self.mode
        return d


@dataclass
class Dimension:
    x1: float
    y1: float
    x2: float
    y2: float
    text: Optional[str] = None

    def to_dict(self):
        d = {"type": "Dimension", "x1": self.x1, "y1": self.y1, "x2": self.x2, "y2": self.y2}
        if self.text:
            d["text"] = self.text
        return d


# --- Rebar ---

@dataclass
class BarGroup:
    count: int
    size: str  # e.g., "#6", "3/4\""

    def to_dict(self):
        return {"count": self.count, "size": self.size}


@dataclass
class RebarEntry:
    pattern: str  # "top", "bot", "sides", "perim", "grid", "uniform"
    bars: List[BarGroup] = field(default_factory=list)

    def to_dict(self):
        return {"pattern": self.pattern, "bars": [b.to_dict() for b in self.bars]}


# --- Section ---

@dataclass
class Shape:
    shape_type: str  # "Rect", "Circle", "Polygon"
    # For Rect
    width: Optional[float] = None
    height: Optional[float] = None
    # For Circle
    diameter: Optional[float] = None
    # For Polygon
    points: Optional[List[Tuple[float, float]]] = None

    def to_dict(self):
        if self.shape_type == "Rect":
            return {"type": "Rect", "width": self.width, "height": self.height}
        elif self.shape_type == "Circle":
            return {"type": "Circle", "diameter": self.diameter}
        elif self.shape_type == "Polygon":
            return {"type": "Polygon", "points": self.points}
        return {}


@dataclass
class ConcreteProperties:
    fc: Optional[float] = None
    cover: Optional[float] = None

    def to_dict(self):
        d = {}
        if self.fc is not None:
            d["fc"] = self.fc
        if self.cover is not None:
            d["cover"] = self.cover
        return d


@dataclass
class Section:
    id: str
    shape: Optional[Shape] = None
    concrete: Optional[ConcreteProperties] = None
    rebar: List[RebarEntry] = field(default_factory=list)
    view: Optional[str] = None  # "section", "longitudinal", "both"

    def to_dict(self):
        d = {"type": "Section", "id": self.id, "properties": {}}
        if self.shape:
            d["properties"]["shape"] = self.shape.to_dict()
        if self.concrete:
            d["properties"]["concrete"] = self.concrete.to_dict()
        if self.rebar:
            d["properties"]["rebar"] = [r.to_dict() for r in self.rebar]
        if self.view:
            d["properties"]["view"] = self.view
        return d


# --- Drawing ---

@dataclass
class Drawing:
    id: str
    elements: List[Union[Rect, Circle, Line, Label, Dimension]] = field(default_factory=list)

    def add(self, element):
        self.elements.append(element)
        return self

    def to_dict(self):
        return {
            "type": "Drawing",
            "id": self.id,
            "elements": [{"Primitive": e.to_dict()} for e in self.elements],
        }


# --- Builders ---

class Beam:
    """Builder for beam sections."""
    def __init__(self, id: str, width: float, height: float):
        self.section = Section(
            id=id,
            shape=Shape("Rect", width=width, height=height),
            concrete=ConcreteProperties(cover=4.0),
        )

    def cover(self, c: float):
        self.section.concrete.cover = c
        return self

    def fc(self, fc: float):
        self.section.concrete.fc = fc
        return self

    def add_top_bars(self, count: int, size: str):
        self.section.rebar.append(RebarEntry("top", [BarGroup(count, size)]))
        return self

    def add_bot_bars(self, count: int, size: str):
        self.section.rebar.append(RebarEntry("bot", [BarGroup(count, size)]))
        return self

    def build(self) -> Section:
        return self.section


class Column:
    """Builder for column sections."""
    def __init__(self, id: str, width: float, height: float):
        self.section = Section(
            id=id,
            shape=Shape("Rect", width=width, height=height),
            concrete=ConcreteProperties(cover=4.0),
        )

    def add_perimeter_bars(self, count: int, size: str):
        self.section.rebar.append(RebarEntry("perim", [BarGroup(count, size)]))
        return self

    def build(self) -> Section:
        return self.section


# --- Export ---

def to_json(elements: List[Union[Section, Drawing]], indent: int = 2) -> str:
    """Serialize a list of sections/drawings to JSON."""
    return json.dumps([e.to_dict() for e in elements], indent=indent)


def to_rcs(elements: List[Union[Section, Drawing]]) -> str:
    """Generate .rcs DSL string from elements (simplified)."""
    lines = []
    for el in elements:
        if isinstance(el, Section):
            s = el
            lines.append(f'section "{s.id}":')
            if s.shape:
                if s.shape.shape_type == "Rect":
                    lines.append(f"    shape rect {s.shape.width} {s.shape.height}")
                elif s.shape.shape_type == "Circle":
                    lines.append(f"    shape circle D {s.shape.diameter}")
            if s.concrete:
                lines.append("    concrete:")
                if s.concrete.fc:
                    lines.append(f"        fc {s.concrete.fc}")
                if s.concrete.cover:
                    lines.append(f"        cover {s.concrete.cover}")
            for r in s.rebar:
                for b in r.bars:
                    lines.append(f"    {r.pattern} {b.count} {b.size}")
            lines.append("")
    return "\n".join(lines)


# --- Example Usage ---
if __name__ == "__main__":
    beam = Beam("V-101", 30, 60).cover(4).add_top_bars(3, "#6").add_bot_bars(2, "#5").build()
    col = Column("C-1", 40, 40).add_perimeter_bars(8, "#8").build()

    print("=== JSON Output ===")
    print(to_json([beam, col]))

    print("\n=== RCS Output ===")
    print(to_rcs([beam, col]))
