subshader "Tlight_m1_Material1" "StandardMesh/Default"
{
	lighting false;
	transparent true;
	blendSrc sourceAlpha; 
	blendDest one;
	depthWrite false;
	alphaTestRef 0.4;
	texture "texture/Effects/tracklight_s";
}

