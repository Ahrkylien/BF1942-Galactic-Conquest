subshader "skyhopper_hull_m1_Material0" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 1 1 1;
	lightingSpecular false;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
      	twosided true;
	texture "texture/Vehicles/Skyhopper/skyhopper";
}

subshader "skyhopper_hull_m1_Material1" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 1 1 1;
	lightingSpecular false;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
	transparent true;
	texture "texture/Vehicles/Skyhopper/skyhopper-glass";
}

