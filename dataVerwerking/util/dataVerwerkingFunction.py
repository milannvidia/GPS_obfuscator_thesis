import pandas as pd
import json
from pyproj import  Transformer

def get_masten(file):
    with open(file, 'r', encoding='utf-8') as f:
        data = json.load(f)
    features_data=data.get('features',[])
    features_df = pd.json_normalize(features_data)

    transformer=Transformer.from_crs('EPSG:31370','EPSG:4326',always_xy=True)

    def transform_coordinates(coords):
        x, y = coords  # Unpack the coordinates
        lon, lat = transformer.transform(x, y)  # Transform to WGS84
        return [lon, lat]
    
    features_df['geometry.coordinates']=features_df['geometry.coordinates'].apply(transform_coordinates)
    return features_df

