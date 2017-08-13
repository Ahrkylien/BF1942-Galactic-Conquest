subshader "LightsaberAllies_m1_Material0" "StandardMesh/Default"
{
	lighting false;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	texture "mods/GCMOD/movies/Lightsaber";
}

subshader "LightsaberAllies_m1_Material1" "StandardMesh/Default"
{
	lighting false;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	transparent true;
	twosided true;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 0.7;
	texture "mods/GCMOD/movies/Lightsaber";
}

subshader "LightsaberAllies_m1_Material2" "StandardMesh/Default"
{
	lighting false;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	texture "mods/GCMOD/movies/Lightsaber";
}

