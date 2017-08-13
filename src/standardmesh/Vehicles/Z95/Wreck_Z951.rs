subshader "Wreck_XWing1_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/Vehicles/Z95/Wreck_XWing_Wing";
}

subshader "Wreck_XWing1_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/Vehicles/Z95/Wreck_XWing_Fus";
}

subshader "xwing_fus_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0.588235 0.588235 0.588235;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
	texture "texture/Vehicles/Z95/Z95_Fus";
}

subshader "z95_fuscan_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	texture "texture/Vehicles/Z95/XWing_Fus";
}

subshader "z95_fuscan_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	texture "texture/Vehicles/Z95/lambda_win";
}

subshader "z95_fuscan_Material5" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	texture "texture/Vehicles/Z95/canopy_steel";
}

