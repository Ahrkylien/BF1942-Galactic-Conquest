subshader "tracklight_Taim_&_Bak_KX9_Laser_m1_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	transparent false;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 0.6;
	texture "texture/Effects/tracklight_s";
}

subshader "tracklight_Taim_&_Bak_KX9_Laser_m1_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	transparent false;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 0.3;
	texture "texture/Effects/tracklight_si";
}

