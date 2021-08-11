# Type-annotated functions for SerialEM commands

def NumMontageTiles() -> int:
    return int(sem.ReportNumMontagePieces())