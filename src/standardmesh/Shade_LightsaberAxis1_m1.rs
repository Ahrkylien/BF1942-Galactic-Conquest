subshader "Shade_LightsaberAxis1_m1_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1.0 1.0 1.0;
	texture "mods/GCMOD/movies/Lightsaber";
}

subshader "Shade_LightsaberAxis1_m1_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0.588235 0.588235 0.588235;
	transparent false;
	twosided true;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 0.0;
	texture "mods/GCMOD/movies/Lightsaber";
}

subshader "Shade_LightsaberAxis1_m1_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1.0 1.0 1.0;
	texture "mods/GCMOD/movies/Lightsaber";
}

