subshader "Tlight_r2_Material0" "StandardMesh/Default"
{
	lighting false;
	transparent true;
	blendSrc sourceAlpha; 
	blendDest one;
	depthWrite false;
	alphaTestRef 0.6;
	texture "texture/Effects/tracklight_s";
}
subshader "Tlight_r2_Material1" "StandardMesh/Default"
{
	lighting false;
	transparent true;
	blendSrc sourceAlpha; 
	blendDest one;
	depthWrite false;
	alphaTestRef 0.3;
	texture "texture/Effects/tracklight_si";
}
