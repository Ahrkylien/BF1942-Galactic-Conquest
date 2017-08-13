subshader "flashProjectile_Material0" "StandardMesh/Default"
{
	lighting false;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	transparent true;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 1.0;
	texture "mods/gcmod/movies/flash";
}

