subshader "Tlight_r1_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 0.6;
	texture "texture/Effects/tracklight_s";
}

subshader "Tlight_r1_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 0.3;
	texture "texture/Effects/tracklight_si";
}

subshader "Tlight_r1_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	transparent true;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 0.6;
	texture "texture/Effects/tracklight_s";
}

