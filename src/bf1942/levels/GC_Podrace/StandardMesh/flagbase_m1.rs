subshader "flagbase_m1_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1 1 1 1;
	blendSrc sourceAlpha;
	blendDest one;
	transparent true;
	alphaTestRef 1.0;
	texture "mods/gcmod/movies/flagbeam";
}

subshader "flagbase_m1_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	texture "texture/flagbase";
}

