subshader "AWing_Tracer_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1 1 1;
	materialSpecular 0 0 0;
	materialSpecularPower 12.5;
	transparent true;
	blendSrc sourceAlpha; 
	blendDest one;
	twosided true;
	texture "texture/AWing/AWing_Tracer";
}

