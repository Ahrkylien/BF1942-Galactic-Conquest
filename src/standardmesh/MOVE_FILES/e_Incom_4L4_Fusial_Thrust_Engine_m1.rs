subshader "e_Incom_4L4_Fusial_Thrust_Engine_m1" "StandardMesh/Default"
{
	lighting false;
	lightingSpecular true;
	materialDiffuse 1 1 1;
	materialSpecular 0.337255 0.337255 0.337255;
	materialSpecularPower 12.5;
	transparent true;
	blendSrc sourceAlpha; 
	blendDest one;
	twosided true;
	alphaTestRef 0.7;
	texture "texture/Effects/e_Incom_4L4_Fusial_Thrust_Engine_I.dds";
}

