subshader "tan_AWing_Fus_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	texture "texture/Vehicles/AWing/awing";
}

subshader "tan_AWing_Fus_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0.588 0.588 0.588;
	materialSpecular 0.9 0.9 0.9;
	materialSpecularPower 72.0;
	transparent true;
	texture "texture/Vehicles/AWing/awing";
}

