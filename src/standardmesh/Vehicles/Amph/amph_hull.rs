subshader "amph_hull_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	texture "texture/vehicles/amph/amphtex";
}

subshader "amph_hull_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	texture "texture/vehicles/amph/floor";
}

subshader "amph_hull_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
      materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
	transparent true;
      twosided true;
	materialDiffuse 0.588 0.588 0.588;
      depthwrite false;
      texture "texture/vehicles/amph/window";
}

