subshader "runner_inside_Material0" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 1.0 1.0 1.0;
	lightingSpecular true;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap false;
	texture "texture/vehicles/runner/floortex";
}

subshader "runner_inside_Material1" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.1 0.1 0.1;
	lightingSpecular true;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap false;
	texture "texture/vehicles/runner/walltex";
}

subshader "runner_inside_Material2" "StandardMesh/Default"
{
	lighting false;
	materialDiffuse 1.0 1.0 1.0;
	lightingSpecular false;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
	texture "texture/vehicles/runner/roof";
}

subshader "runner_inside_Material3" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.1 0.1 0.1;
	lightingSpecular true;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap false;
	texture "texture/vehicles/runner/wall2";
}

subshader "runner_inside_Material4" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 1.0 1.0 1.0;
	lightingSpecular false;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	envmap true;
        texture "texture/vehicles/runner/frontpannel";
}

