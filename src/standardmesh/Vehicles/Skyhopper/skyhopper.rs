subshader "skyhopper_hull_m1_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0.588235 0.588235 0.588235;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
      twosided true;
	texture "texture/Vehicles/Skyhopper/skyhopper";
}

subshader "skyhopper_hull_m1_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0.588235 0.588235 0.588235;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	transparent true;
	envmap true;
	texture "texture/Vehicles/Skyhopper/skyhopper-glass";
}

