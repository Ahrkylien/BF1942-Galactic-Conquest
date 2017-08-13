subshader "flagbase_nocol_m1_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	transparent true;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 1.0;
	texture "texture/flagbeam";
}

