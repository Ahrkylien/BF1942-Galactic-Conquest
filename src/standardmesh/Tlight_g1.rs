subshader "Tlight_g1_Material0" "StandardMesh/Default"
{
	lighting false;
	transparent true;
	blendSrc sourceAlpha; 
	blendDest one;
	depthWrite false;
	alphaTestRef 0.6;
	texture "texture/tracklight_sg";
}
subshader "Tlight_g1_Material1" "StandardMesh/Default"
{
	lighting false;
	transparent true;
	blendSrc sourceAlpha; 
	blendDest one;
	depthWrite false;
	alphaTestRef 0.3;
	texture "texture/tracklight_si";
}
