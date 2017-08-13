subshader "tan_AWing_Fus_Material0" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.588235 0.588235 0.588235;
	lightingSpecular true;
	materialSpecular 0.5 0.5 0.5;
	materialSpecularPower 1.0;
        twosided false;
	envmap true;
	texture "texture/Vehicles/AWing/awing";
}

subshader "tan_AWing_Fus_Material1" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.588235 0.588235 0.588235;
	lightingSpecular true;
	materialSpecular 0.5 0.5 0.5;
	materialSpecularPower 1.0;
        twosided false;
	envmap true;
	transparent true;
	texture "texture/Vehicles/AWing/awing";
}

