subshader "UseTheForceshield_Material0" "StandardMesh/Default"
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


