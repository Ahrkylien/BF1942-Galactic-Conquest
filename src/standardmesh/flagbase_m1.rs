subshader "flagbase_m1_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	transparent true;
      depthWrite false;
	alphaTestRef 0.7;
	texture "mods/gcmod/movies/flagbeam+alpha.tga";
}

subshader "flagbase_m1_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	texture "texture/flagbase";
}

