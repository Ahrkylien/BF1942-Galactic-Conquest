subshader "skyhopper_low_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	texture "texture/Vehicles/Skyhopper/skyhopper";
}

subshader "skyhopper_low_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0.588235 0.588235 0.588235;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
	texture "texture/Vehicles/Skyhopper/skyhopper-glass";
}

subshader "skyhopper_low_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	texture "texture/Vehicles/Skyhopper/skyhopper_gun";
}

