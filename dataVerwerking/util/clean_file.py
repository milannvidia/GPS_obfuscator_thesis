from geopy.distance import geodesic # type: ignore

def filter_by_distance(
    location: tuple[float, float], privacyBlob: tuple[float, float, float]
) -> bool:
    return (
        float(
            geodesic(
                (location[0], location[1]), (privacyBlob[0], privacyBlob[1])
            ).meters  # type: ignore
        )
        > privacyBlob[2]
    )
